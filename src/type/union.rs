use super::Type;
use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn into_union(&mut self, types: Vec<Type<'a>>) -> Type<'a> {
    match types.len() {
      0 => Type::Undefined,
      1 => types.into_iter().next().unwrap(),
      _ => Type::Union(self.allocator.alloc(types)),
    }
  }
}
