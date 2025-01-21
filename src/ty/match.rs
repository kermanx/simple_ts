use oxc::semantic::SymbolId;
use rustc_hash::FxHashMap;

use super::{intersection::IntersectionTypeBuilder, unresolved::UnresolvedType, Ty};
use crate::Analyzer;

enum MatchResult<'a> {
  Unmatched,
  Matched,
  Inferred(FxHashMap<SymbolId, Ty<'a>>),
  Multiple(Vec<MatchResult<'a>>),
  Error,
}

impl<'a> Analyzer<'a> {
  /// `Target extends Pattern`
  /// Returns
  /// - `None` if the types do not match
  /// - `Some` with a map of `infer T` in pattern
  pub fn match_types(&mut self, target: Ty<'a>, pattern: Ty<'a>) -> MatchResult<'a> {
    match (target, pattern) {
      (target, pattern) if target == pattern => MatchResult::Matched,

      (target, Ty::Unresolved(UnresolvedType::InferType(s))) => MatchResult::Inferred({
        let mut map = FxHashMap::default();
        map.insert(s, target);
        map
      }),

      (Ty::Any | Ty::Error, _) => {
        MatchResult::Multiple(vec![MatchResult::Unmatched, MatchResult::Matched])
      }
      (_, Ty::Any | Ty::Error | Ty::Unknown) => MatchResult::Matched,

      (Ty::Undefined, Ty::Void) => MatchResult::Matched,

      (Ty::StringLiteral(_), Ty::String) => MatchResult::Matched,
      (Ty::NumericLiteral(_), Ty::Number) => MatchResult::Matched,
      (Ty::BigIntLiteral(_), Ty::BigInt) => MatchResult::Matched,
      (Ty::BooleanLiteral(_), Ty::Boolean) => MatchResult::Matched,
      (Ty::UniqueSymbol(_), Ty::Symbol) => MatchResult::Matched,

      (Ty::Record(target), Ty::Record(pattern)) => todo!(),
      (Ty::Function(target), Ty::Function(pattern)) => todo!(),
      (Ty::Constructor(target), Ty::Constructor(pattern)) => todo!(),
      (Ty::Namespace(target), Ty::Namespace(pattern)) => todo!(),

      (Ty::Union(target), Ty::Union(pattern)) => {
        todo!()
      }
      (Ty::Union(target), pattern) => {
        let mut results = Vec::new();
        target.for_each(|ty| results.push(self.match_types(ty, pattern)));
        MatchResult::Multiple(results)
      }
      (Ty::Intersection(target), Ty::Intersection(pattern)) => {
        todo!()
      }
      (Ty::Intersection(target), pattern) => {
        let mut error = false;
        let mut matched: Option<Option<FxHashMap<SymbolId, IntersectionTypeBuilder<'a>>>> = None;
        target.for_each(|ty| {
          let result = self.match_types(ty, pattern);
          match result {
            MatchResult::Unmatched => {}
            MatchResult::Matched => {
              matched.get_or_insert_with(Default::default);
            }
            MatchResult::Inferred(map) => {
              let inferred =
                matched.get_or_insert_with(Default::default).get_or_insert_with(Default::default);
              for (s, t) in map {
                let builder = inferred.entry(s).or_insert_with(Default::default);
                builder.add(t);
              }
            }
            MatchResult::Multiple(results) => todo!(),
            MatchResult::Error => error = true,
          }
        });
        if error {
          MatchResult::Error
        } else if let Some(matched) = matched {
          if let Some(inferred) = matched {
            let mut map = FxHashMap::default();
            for (s, builder) in inferred {
              map.insert(s, builder.build(self.allocator));
            }
            MatchResult::Inferred(map)
          } else {
            MatchResult::Matched
          }
        } else {
          MatchResult::Unmatched
        }
      }

      (Ty::Generic(_) | Ty::Intrinsic(_), _) => MatchResult::Error,
      (_, Ty::Generic(_) | Ty::Intrinsic(_)) => MatchResult::Error,

      _ => MatchResult::Unmatched,
    }
  }
}
