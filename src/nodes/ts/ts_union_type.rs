use oxc::ast::ast::TSUnionType;

use crate::{
  ty::{union::into_union, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn resolve_union_type(&mut self, node: &'a TSUnionType<'a>) -> Ty<'a> {
    let mut types = vec![];
    for node in &node.types {
      types.push(self.resolve_type(node));
    }
    into_union(self.allocator, types)
  }
}
