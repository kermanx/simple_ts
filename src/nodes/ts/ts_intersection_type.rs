use crate::{
  ty::{intersection::IntersectionTypeBuilder, Ty},
  Analyzer,
};
use oxc::ast::ast::TSIntersectionType;

impl<'a> Analyzer<'a> {
  pub fn resolve_intersection_type(&mut self, node: &'a TSIntersectionType<'a>) -> Ty<'a> {
    let mut builder = IntersectionTypeBuilder::default();

    for type_node in &node.types {
      let ty = self.resolve_type(type_node);
      builder.add(ty);
    }

    builder.build(self.allocator)
  }
}
