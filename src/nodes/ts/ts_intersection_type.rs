use oxc::ast::ast::TSIntersectionType;

use crate::{Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_intersection_type(&mut self, node: &'a TSIntersectionType<'a>) -> Ty<'a> {
    let mut types = vec![];
    for node in &node.types {
      types.push(self.resolve_type(node));
    }
    self.into_intersection(types)
  }
}
