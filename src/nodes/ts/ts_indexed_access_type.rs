use oxc::ast::ast::TSIndexedAccessType;

use crate::{Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_indexed_access_type(&mut self, node: &'a TSIndexedAccessType<'a>) -> Ty<'a> {
    let object_type = self.resolve_type(&node.object_type);
    let index_type = self.resolve_type(&node.index_type);
    let index_type = self.to_property_key(index_type);
    self.get_property(object_type, index_type)
  }
}
