use oxc::semantic::SymbolId;
use rustc_hash::FxHashMap;

use super::{callable::CallableType, unresolved::UnresolvedType, Ty};
use crate::Analyzer;

pub enum MatchResult<'a> {
  Error,
  Unmatched,
  Matched,
  Inferred(FxHashMap<SymbolId, Ty<'a>>),
}

impl<'a> From<bool> for MatchResult<'a> {
  fn from(b: bool) -> Self {
    if b {
      MatchResult::Matched
    } else {
      MatchResult::Unmatched
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn match_types_with_dispatch(
    &mut self,
    target: Ty<'a>,
    pattern: Ty<'a>,
  ) -> Vec<MatchResult<'a>> {
    match (target, pattern) {
      (_, Ty::Any | Ty::Error | Ty::Unknown) => vec![MatchResult::Matched],
      (Ty::Any | Ty::Error, _) => vec![MatchResult::Matched, MatchResult::Unmatched],
      (Ty::Union(u), _) => {
        let mut results = Vec::new();
        u.for_each(|ty| results.extend(self.match_types_with_dispatch(ty, pattern)));
        results
      }
      _ => vec![self.match_covariant_types(target, pattern)],
    }
  }

  fn match_covariant_types(&mut self, target: Ty<'a>, pattern: Ty<'a>) -> MatchResult<'a> {
    match (target, pattern) {
      (Ty::Generic(_) | Ty::Intrinsic(_), _) => MatchResult::Error,
      (_, Ty::Generic(_) | Ty::Intrinsic(_)) => MatchResult::Error,
      (_, Ty::Namespace(_)) | (Ty::Namespace(_), _) => MatchResult::Error,

      (target, pattern) if target == pattern => MatchResult::Matched,

      (matched, Ty::Unresolved(UnresolvedType::InferType(s)))
      | (Ty::Unresolved(UnresolvedType::InferType(s)), matched) => MatchResult::Inferred({
        let mut map = FxHashMap::default();
        map.insert(s, matched);
        map
      }),
      (Ty::Unresolved(_), _) | (_, Ty::Unresolved(_)) => todo!(),

      (_, Ty::Never) => MatchResult::Unmatched,
      (Ty::Error | Ty::Any | Ty::Never, _) => MatchResult::Matched,
      (_, Ty::Error | Ty::Any | Ty::Unknown) => MatchResult::Matched,
      (Ty::Unknown, _) => MatchResult::Unmatched,

      (Ty::Union(target), Ty::Union(_)) => {
        let mut error = false;
        let mut unmatched = false;
        let mut inferred = FxHashMap::<SymbolId, Vec<Ty<'a>>>::default();

        target.for_each(|ty| match self.match_covariant_types(ty, pattern) {
          MatchResult::Error => error = true,
          MatchResult::Unmatched => unmatched = true,
          MatchResult::Matched => {}
          MatchResult::Inferred(i) => {
            for (s, t) in i {
              inferred.entry(s).or_insert_with(Default::default).push(t);
            }
          }
        });

        if error {
          MatchResult::Error
        } else if unmatched {
          MatchResult::Unmatched
        } else {
          MatchResult::Inferred(
            inferred.into_iter().map(|(s, types)| (s, self.into_union(types))).collect(),
          )
        }
      }
      (Ty::Union(_), _) => MatchResult::Unmatched,
      (target, Ty::Union(pattern)) => {
        let mut error = false;
        let mut matched = false;
        let mut inferred = FxHashMap::default();

        pattern.for_each(|pattern| match self.match_covariant_types(target, pattern) {
          MatchResult::Error => error = true,
          MatchResult::Unmatched => {}
          MatchResult::Matched => matched = true,
          MatchResult::Inferred(i) => {
            matched = true;
            inferred.extend(i);
          }
        });

        if error {
          MatchResult::Error
        } else if matched {
          MatchResult::Inferred(inferred)
        } else {
          MatchResult::Unmatched
        }
      }

      (Ty::Intersection(_), Ty::Intersection(pattern)) => {
        let mut error = false;
        let mut matched = true;
        let mut inferred = FxHashMap::<SymbolId, Vec<Ty<'a>>>::default();

        pattern.for_each(|pattern| match self.match_covariant_types(target, pattern) {
          MatchResult::Error => error = true,
          MatchResult::Unmatched => matched = false,
          MatchResult::Matched => {}
          MatchResult::Inferred(map) => {
            for (s, t) in map {
              inferred.entry(s).or_insert_with(Default::default).push(t);
            }
          }
        });

        if error {
          MatchResult::Error
        } else if matched {
          MatchResult::Inferred(
            inferred.into_iter().map(|(s, types)| (s, self.into_intersection(types))).collect(),
          )
        } else {
          MatchResult::Unmatched
        }
      }
      (_, Ty::Intersection(_)) => MatchResult::Unmatched,
      (Ty::Intersection(target), pattern) => {
        let mut error = false;
        let mut matched: Option<Option<FxHashMap<SymbolId, Vec<Ty<'a>>>>> = None;
        target.for_each(|target| match self.match_covariant_types(target, pattern) {
          MatchResult::Error => error = true,
          MatchResult::Unmatched => {}
          MatchResult::Matched => {
            matched.get_or_insert_with(Default::default);
          }
          MatchResult::Inferred(map) => {
            let inferred =
              matched.get_or_insert_with(Default::default).get_or_insert_with(Default::default);
            for (s, t) in map {
              inferred.entry(s).or_insert_with(Default::default).push(t);
            }
          }
        });
        if error {
          MatchResult::Error
        } else if let Some(matched) = matched {
          if let Some(inferred) = matched {
            MatchResult::Inferred(
              inferred.into_iter().map(|(s, types)| (s, self.into_intersection(types))).collect(),
            )
          } else {
            MatchResult::Matched
          }
        } else {
          MatchResult::Unmatched
        }
      }

      (Ty::Instance(target), Ty::Instance(pattern)) => {
        // See https://github.com/Microsoft/TypeScript/wiki/FAQ#structural-vs-instantiation-based-inference.
        // 1. Instantiation based inference
        if target.generic == pattern.generic {
          'instantiation_based_inference: {
            let mut inferred = FxHashMap::default();
            for (target, pattern) in target.args.iter().zip(pattern.args.iter()) {
              match self.match_covariant_types(*target, *pattern) {
                MatchResult::Error | MatchResult::Unmatched => break 'instantiation_based_inference,
                MatchResult::Matched => {}
                MatchResult::Inferred(map) => inferred.extend(map),
              }
            }
            return MatchResult::Inferred(inferred);
          }
        }

        // 2. Structural inference
        let target = self.unwrap_generic_instance(target);
        let pattern = self.unwrap_generic_instance(pattern);
        self.match_covariant_types(target, pattern)
      }
      (Ty::Instance(target), pattern) => {
        let target = self.unwrap_generic_instance(target);
        self.match_covariant_types(target, pattern)
      }
      (target, Ty::Instance(pattern)) => {
        let pattern = self.unwrap_generic_instance(pattern);
        self.match_covariant_types(target, pattern)
      }

      (Ty::Record(target), Ty::Record(pattern)) => todo!(),
      (Ty::Object, Ty::Record(pattern)) => MatchResult::from(pattern.is_empty()),
      (Ty::Record(_), Ty::Object) => MatchResult::Matched,
      (_, Ty::Record(_)) | (Ty::Record(_), _) => MatchResult::Unmatched,

      (Ty::Interface(target), Ty::Interface(pattern)) => todo!(),
      (Ty::Object, Ty::Interface(pattern)) => MatchResult::from(pattern.is_empty()),
      (Ty::Interface(_), Ty::Object) => MatchResult::Matched,
      (_, Ty::Interface(_)) | (Ty::Interface(_), _) => MatchResult::Unmatched,

      (Ty::Function(target), Ty::Function(pattern)) => self.match_callable_types(target, pattern),
      (Ty::Function(_), Ty::Object) => MatchResult::Matched,
      (Ty::Function(_), _) | (_, Ty::Function(_)) => MatchResult::Unmatched,

      (Ty::Constructor(target), Ty::Constructor(pattern)) => {
        self.match_callable_types(target, pattern)
      }
      (Ty::Constructor(_), Ty::Object) => MatchResult::Matched,
      (Ty::Constructor(_), _) | (_, Ty::Constructor(_)) => MatchResult::Unmatched,

      (Ty::Undefined, Ty::Void) => MatchResult::Matched,
      (Ty::Undefined, _) => MatchResult::Unmatched,
      (_, Ty::Void) => MatchResult::Unmatched,
      (Ty::Void, _) => MatchResult::Unmatched,

      (Ty::Null, _) | (_, Ty::Null) => MatchResult::Unmatched,
      (_, Ty::Object) | (Ty::Object, _) => MatchResult::Unmatched,

      (Ty::String | Ty::StringLiteral(_), pattern) => MatchResult::from(pattern == Ty::String),
      (Ty::Number | Ty::NumericLiteral(_), pattern) => MatchResult::from(pattern == Ty::Number),
      (Ty::BigInt | Ty::BigIntLiteral(_), pattern) => MatchResult::from(pattern == Ty::BigInt),
      (Ty::Boolean | Ty::BooleanLiteral(_), pattern) => MatchResult::from(pattern == Ty::Boolean),
      (Ty::Symbol | Ty::UniqueSymbol(_), pattern) => MatchResult::from(pattern == Ty::Symbol),
    }
  }

  fn match_contravariant_types(&mut self, target: Ty<'a>, pattern: Ty<'a>) -> MatchResult<'a> {
    self.match_covariant_types(pattern, target)
  }

  /// Contravariant first, then covariant
  fn match_bivariant_types(&mut self, target: Ty<'a>, pattern: Ty<'a>) -> MatchResult<'a> {
    match self.match_contravariant_types(target, pattern) {
      MatchResult::Unmatched => self.match_covariant_types(target, pattern),
      result => result,
    }
  }

  fn match_parameter_types(
    &mut self,
    bivariant: bool,
    target: Ty<'a>,
    pattern: Ty<'a>,
  ) -> MatchResult<'a> {
    if bivariant {
      self.match_bivariant_types(target, pattern)
    } else {
      self.match_covariant_types(target, pattern)
    }
  }

  fn match_callable_types<const CTOR: bool>(
    &mut self,
    target: &'a CallableType<'a, CTOR>,
    pattern: &'a CallableType<'a, CTOR>,
  ) -> MatchResult<'a> {
    // Step1: Match type parameters
    // FIXME: Do not match unused type parameters
    if target.type_params.len() != pattern.type_params.len() {
      return MatchResult::Unmatched;
    }
    for (target, pattern) in target.type_params.iter().zip(pattern.type_params.iter()) {
      if target.constraint == pattern.constraint {
        continue;
      }
      // Note: Contravariance - `pattern.constraint extends target.constraint`
      match self.match_covariant_types(
        pattern.constraint.unwrap_or(Ty::Unknown),
        target.constraint.unwrap_or(Ty::Unknown),
      ) {
        MatchResult::Error => return MatchResult::Error,
        MatchResult::Unmatched => return MatchResult::Unmatched,
        MatchResult::Matched => {}
        MatchResult::Inferred(_) => {
          // Somehow in TypeScript this is ignored
        }
      }
    }

    let bivariant = pattern.bivariant;
    let mut inferred = FxHashMap::default();

    // Step2: Match this type
    if let (Some(target), Some(pattern)) = (target.this_param, pattern.this_param) {
      match self.match_parameter_types(bivariant, pattern, target) {
        MatchResult::Error => return MatchResult::Error,
        MatchResult::Unmatched => return MatchResult::Unmatched,
        MatchResult::Matched => {}
        MatchResult::Inferred(map) => inferred.extend(map),
      }
    }

    // Step3: Match parameters
    for (index, (target_optional, mut target)) in target.params.iter().enumerate() {
      if let Some((pattern_optional, mut pattern)) = pattern.params.get(index) {
        if target_optional != pattern_optional {
          target = self.get_optional_type(*target_optional, target);
          pattern = self.get_optional_type(*pattern_optional, pattern);
        }
        match self.match_parameter_types(bivariant, pattern, target) {
          MatchResult::Error => return MatchResult::Error,
          MatchResult::Unmatched => return MatchResult::Unmatched,
          MatchResult::Matched => {}
          MatchResult::Inferred(map) => inferred.extend(map),
        }
      } else if *target_optional {
        // Optional parameter, matched
      } else {
        // TODO: Check rest parameter
        return MatchResult::Unmatched;
      }
    }

    // Step4: Match rest parameter
    // TODO: Check rest parameter

    // Step5: Match return type
    match self.match_covariant_types(target.return_type, pattern.return_type) {
      MatchResult::Error => return MatchResult::Error,
      MatchResult::Unmatched => return MatchResult::Unmatched,
      MatchResult::Matched => {}
      MatchResult::Inferred(map) => inferred.extend(map),
    }

    MatchResult::Inferred(inferred)
  }
}
