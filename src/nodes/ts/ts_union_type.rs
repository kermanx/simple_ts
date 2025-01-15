use crate::{
  ty::{union::into_union, Ty},
  Analyzer,
};
use oxc::ast::ast::TSUnionType;

impl<'a> Analyzer<'a> {
  pub fn resolve_union_type(&mut self, node: &'a TSUnionType<'a>) -> Option<Ty<'a>> {
    let mut types = vec![];
    for node in &node.types {
      types.push(self.resolve_type(node)?);
    }
    Some(into_union(self.allocator, types))
  }
}
