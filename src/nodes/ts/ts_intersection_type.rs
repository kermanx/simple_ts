use crate::{
  ty::{intersection::into_intersection, Ty},
  Analyzer,
};
use oxc::ast::ast::TSIntersectionType;

impl<'a> Analyzer<'a> {
  pub fn resolve_intersection_type(&mut self, node: &'a TSIntersectionType<'a>) -> Ty<'a> {
    let mut types = vec![];
    for node in &node.types {
      types.push(self.resolve_type(node));
    }
    into_intersection(self.allocator, types)
  }
}
