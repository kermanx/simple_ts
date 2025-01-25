use std::{collections::hash_map::Entry, hash::Hash};

use oxc::semantic::SymbolId;
use rustc_hash::FxHashMap;

use super::{callable::CallableType, record::KeyedPropertyMap, unresolved::UnresolvedType, Ty};
use crate::Analyzer;

pub enum MatchResult<'a> {
  Error,
  Unmatched,
  Matched,
  Inferred(FxHashMap<SymbolId, (i32, Ty<'a>)>),
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

impl<'a> MatchResult<'a> {
  pub fn matched(self) -> bool {
    matches!(self, MatchResult::Matched | MatchResult::Inferred(_))
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
      _ => vec![self.match_covariant_types(1, target, pattern)],
    }
  }

  pub fn match_covariant_types(
    &mut self,
    specificity: i32,
    target: Ty<'a>,
    pattern: Ty<'a>,
  ) -> MatchResult<'a> {
    match (target, pattern) {
      (Ty::Generic(_) | Ty::Intrinsic(_), _) => MatchResult::Error,
      (_, Ty::Generic(_) | Ty::Intrinsic(_)) => MatchResult::Error,
      (_, Ty::Namespace(_)) | (Ty::Namespace(_), _) => MatchResult::Error,

      (target, pattern) if target == pattern => MatchResult::Matched,

      (matched, Ty::Unresolved(UnresolvedType::InferType(s))) => {
        MatchResult::Inferred(FxHashMap::from_iter([(s, (specificity, matched))]))
      }
      (Ty::Unresolved(UnresolvedType::InferType(s)), matched) => {
        MatchResult::Inferred(FxHashMap::from_iter([(s, (-specificity, matched))]))
      }

      (_, Ty::Never) => MatchResult::Unmatched,
      (Ty::Error | Ty::Any | Ty::Never, _) => MatchResult::Matched,
      (_, Ty::Error | Ty::Any | Ty::Unknown) => MatchResult::Matched,
      (Ty::Unknown, _) => MatchResult::Unmatched,

      (Ty::Unresolved(target), Ty::Unresolved(pattern)) => match (target, pattern) {
        (UnresolvedType::Placeholder(_), _) | (_, UnresolvedType::Placeholder(_)) => {
          MatchResult::Unmatched
        }
        _ => todo!(),
      },
      (Ty::Unresolved(_), _) | (_, Ty::Unresolved(_)) => todo!(),

      (Ty::Union(target), pattern) => {
        let mut error = false;
        let mut unmatched = false;
        let mut inferred = FxHashMap::<SymbolId, Vec<(i32, Ty<'a>)>>::default();

        target.for_each(|ty| match self.match_covariant_types(specificity, ty, pattern) {
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
            inferred
              .into_iter()
              .map(|(s, types)| (s, self.into_union_with_specificity(types)))
              .collect(),
          )
        }
      }
      (target, Ty::Union(pattern)) => {
        let mut builder = BuilderBySpecificity::default();

        pattern.for_each(|pattern| {
          let result = self.match_covariant_types(specificity, target, pattern);
          builder.add(result);
        });

        builder.into_result()
      }

      (Ty::Intersection(_), Ty::Intersection(pattern)) => {
        let mut error = false;
        let mut matched = true;
        let mut inferred = FxHashMap::<SymbolId, Vec<(i32, Ty<'a>)>>::default();

        pattern.for_each(|pattern| {
          match self.match_covariant_types(specificity, target, pattern) {
            MatchResult::Error => error = true,
            MatchResult::Unmatched => matched = false,
            MatchResult::Matched => {}
            MatchResult::Inferred(map) => {
              for (s, t) in map {
                inferred.entry(s).or_insert_with(Default::default).push(t);
              }
            }
          }
        });

        if error {
          MatchResult::Error
        } else if matched {
          MatchResult::Inferred(
            inferred
              .into_iter()
              .map(|(s, types)| (s, self.into_intersection_with_specificity(types)))
              .collect(),
          )
        } else {
          MatchResult::Unmatched
        }
      }
      (_, Ty::Intersection(_)) => MatchResult::Unmatched,
      (Ty::Intersection(target), pattern) => {
        let mut error = false;
        let mut matched = false;
        let mut inferred = FxHashMap::<SymbolId, Vec<(i32, Ty<'a>)>>::default();
        target.for_each(|target| match self.match_covariant_types(specificity, target, pattern) {
          MatchResult::Error => error = true,
          MatchResult::Unmatched => {}
          MatchResult::Matched => matched = true,
          MatchResult::Inferred(map) => {
            matched = true;
            for (s, t) in map {
              inferred.entry(s).or_insert_with(Default::default).push(t);
            }
          }
        });
        if error {
          MatchResult::Error
        } else if matched {
          MatchResult::Inferred(
            inferred
              .into_iter()
              .map(|(s, types)| (s, self.into_intersection_with_specificity(types)))
              .collect(),
          )
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
              match self.match_covariant_types(specificity + 1, *target, *pattern) {
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
        self.match_covariant_types(specificity, target, pattern)
      }
      (Ty::Instance(target), pattern) => {
        let target = self.unwrap_generic_instance(target);
        self.match_covariant_types(specificity, target, pattern)
      }
      (target, Ty::Instance(pattern)) => {
        let pattern = self.unwrap_generic_instance(pattern);
        self.match_covariant_types(specificity, target, pattern)
      }

      (Ty::Record(target), Ty::Record(pattern)) => {
        let mut builder = BuilderBySpecificity::default();

        self.match_record_keyed_properties(
          &mut builder,
          specificity + 1,
          &target.string_keyed,
          &pattern.string_keyed,
        );
        self.match_record_keyed_properties(
          &mut builder,
          specificity + 1,
          &target.symbol_keyed,
          &pattern.symbol_keyed,
        );

        // TODO: Check mapped properties

        builder.into_result()
      }
      (Ty::Object, Ty::Record(pattern)) => MatchResult::from(pattern.is_empty()),
      (Ty::Record(_), Ty::Object) => MatchResult::Matched,
      (_, Ty::Record(_)) | (Ty::Record(_), _) => MatchResult::Unmatched,

      (Ty::Interface(target), Ty::Interface(pattern)) => todo!(),
      (Ty::Object, Ty::Interface(pattern)) => MatchResult::from(pattern.is_empty()),
      (Ty::Interface(_), Ty::Object) => MatchResult::Matched,
      (_, Ty::Interface(_)) | (Ty::Interface(_), _) => MatchResult::Unmatched,

      (Ty::Tuple(target), Ty::Tuple(pattern)) => todo!(),
      (_, Ty::Tuple(_)) | (Ty::Tuple(_), _) => MatchResult::Unmatched,

      (Ty::Function(target), Ty::Function(pattern)) => {
        self.match_callable_types(specificity, target, pattern)
      }
      (Ty::Function(_), Ty::Object) => MatchResult::Matched,
      (Ty::Function(_), _) | (_, Ty::Function(_)) => MatchResult::Unmatched,

      (Ty::Constructor(target), Ty::Constructor(pattern)) => {
        self.match_callable_types(specificity, target, pattern)
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

  fn match_contravariant_types(
    &mut self,
    specificity: i32,
    target: Ty<'a>,
    pattern: Ty<'a>,
  ) -> MatchResult<'a> {
    self.match_covariant_types(specificity, pattern, target)
  }

  /// Contravariant first, then covariant
  fn match_bivariant_types(
    &mut self,
    specificity: i32,
    target: Ty<'a>,
    pattern: Ty<'a>,
  ) -> MatchResult<'a> {
    match self.match_contravariant_types(specificity, target, pattern) {
      MatchResult::Unmatched => self.match_covariant_types(specificity, target, pattern),
      result => result,
    }
  }

  fn match_parameter_types(
    &mut self,
    bivariant: bool,
    specificity: i32,
    target: Ty<'a>,
    pattern: Ty<'a>,
  ) -> MatchResult<'a> {
    if bivariant {
      self.match_bivariant_types(specificity, target, pattern)
    } else {
      self.match_contravariant_types(specificity, target, pattern)
    }
  }

  fn match_callable_types<const CTOR: bool>(
    &mut self,
    specificity: i32,
    target: &'a CallableType<'a, CTOR>,
    pattern: &'a CallableType<'a, CTOR>,
  ) -> MatchResult<'a> {
    let specificity = specificity + 1;

    // Step1: Match type parameters
    // FIXME: Do not match unused type parameters
    if target.type_params.len() != pattern.type_params.len() {
      return MatchResult::Unmatched;
    }
    let target_scope = self.type_scopes.create_scope();
    let pattern_scope = self.type_scopes.create_scope();
    for (target, pattern) in target.type_params.iter().zip(pattern.type_params.iter()) {
      if target.constraint == pattern.constraint {
        continue;
      }
      // Note: Contravariance - `pattern.constraint extends target.constraint`
      let target_ty =
        target.constraint.map_or(Ty::Unknown, |ty| self.resolve_ctx_ty(target_scope, ty));
      let pattern_ty =
        pattern.constraint.map_or(Ty::Unknown, |ty| self.resolve_ctx_ty(pattern_scope, ty));
      match self.match_contravariant_types(specificity, target_ty, pattern_ty) {
        MatchResult::Error => return MatchResult::Error,
        MatchResult::Unmatched => return MatchResult::Unmatched,
        MatchResult::Matched => {}
        MatchResult::Inferred(_) => {
          // Somehow in TypeScript this is ignored
        }
      }

      let placeholder = self.alloc_placeholder_type();
      self.type_scopes.insert_on_scope(target_scope, target.symbol_id, placeholder);
      self.type_scopes.insert_on_scope(pattern_scope, pattern.symbol_id, placeholder);
    }

    let bivariant = pattern.is_method;
    let mut inferred = FxHashMap::default();

    // Step2: Match this type
    if let (Some(target), Some(pattern)) = (target.this_param, pattern.this_param) {
      let target_ty = self.resolve_ctx_ty(target_scope, target);
      let pattern_ty = self.resolve_ctx_ty(pattern_scope, pattern);
      match self.match_parameter_types(bivariant, specificity, target_ty, pattern_ty) {
        MatchResult::Error => return MatchResult::Error,
        MatchResult::Unmatched => return MatchResult::Unmatched,
        MatchResult::Matched => {}
        MatchResult::Inferred(map) => inferred.extend(map),
      }
    }

    // Step3: Match parameters
    for (index, (target_optional, target)) in target.params.iter().enumerate() {
      if let Some((pattern_optional, pattern)) = pattern.params.get(index) {
        let mut target_ty = self.resolve_ctx_ty(target_scope, *target);
        let mut pattern_ty = self.resolve_ctx_ty(pattern_scope, *pattern);
        if target_optional != pattern_optional {
          target_ty = self.get_optional_type(*target_optional, target_ty);
          pattern_ty = self.get_optional_type(*pattern_optional, pattern_ty);
        }
        match self.match_parameter_types(bivariant, specificity, pattern_ty, target_ty) {
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
    {
      let target_ty = self.resolve_ctx_ty(target_scope, target.return_type);
      let pattern_ty = self.resolve_ctx_ty(pattern_scope, pattern.return_type);
      match self.match_covariant_types(specificity, target_ty, pattern_ty) {
        MatchResult::Error => return MatchResult::Error,
        MatchResult::Unmatched => return MatchResult::Unmatched,
        MatchResult::Matched => {}
        MatchResult::Inferred(map) => inferred.extend(map),
      }
    }

    MatchResult::Inferred(inferred)
  }

  fn match_record_keyed_properties<K: Eq + Hash>(
    &mut self,
    builder: &mut BuilderBySpecificity<'a>,
    specificity: i32,
    target: &KeyedPropertyMap<'a, K>,
    pattern: &KeyedPropertyMap<'a, K>,
  ) {
    for (key, target) in &target.0 {
      if let Some(pattern) = pattern.0.get(key) {
        let result = self.match_covariant_types(specificity, target.value, pattern.value);
        builder.add(result);
      } else {
        builder.add(MatchResult::Unmatched);
      }
    }
  }
}

#[derive(Debug, Default)]
struct BuilderBySpecificity<'a> {
  error: bool,
  matched: bool,
  inferred: FxHashMap<SymbolId, (i32, Ty<'a>)>,
}

impl<'a> BuilderBySpecificity<'a> {
  fn insert(&mut self, symbol: SymbolId, specificity: i32, ty: Ty<'a>) {
    match self.inferred.entry(symbol) {
      Entry::Occupied(mut entry) => {
        let (prev_specificity, _) = *entry.get();
        if prev_specificity.abs() < specificity.abs() {
          entry.insert((specificity, ty));
        }
      }
      Entry::Vacant(entry) => {
        entry.insert((specificity, ty));
      }
    }
  }

  fn add(&mut self, result: MatchResult<'a>) {
    match result {
      MatchResult::Error => self.error = true,
      MatchResult::Unmatched => {}
      MatchResult::Matched => self.matched = true,
      MatchResult::Inferred(i) => {
        self.matched = true;
        for (symbol, (specificity, ty)) in i {
          self.insert(symbol, specificity, ty);
        }
      }
    }
  }

  fn into_result(self) -> MatchResult<'a> {
    if self.error {
      MatchResult::Error
    } else if self.matched {
      MatchResult::Inferred(self.inferred)
    } else {
      MatchResult::Unmatched
    }
  }
}
