use oxc::ast::ast::TSUnionType;

use crate::{ty::Ty, Analyzer};

impl<'a> Analyzer<'a> {
  pub fn resolve_union_type(&mut self, node: &'a TSUnionType<'a>) -> Ty<'a> {
    let mut types = vec![];
    for node in &node.types {
      types.push(self.resolve_type(node));
    }
    self.into_union(types)
  }
}
