use oxc::ast::ast::TSArrayType;

use crate::{
  Analyzer,
  ty::{
    Ty,
    tuple::{TupleElement, TupleType},
  },
};

impl<'a> Analyzer<'a> {
  pub fn resolve_array_type(&mut self, node: &'a TSArrayType<'a>, readonly: bool) -> Ty<'a> {
    Ty::Tuple(self.allocator.alloc(TupleType {
      elements: self.allocator.alloc_slice([TupleElement {
        name: None,
        spread: true,
        optional: false,
        ty: self.resolve_type(&node.element_type),
      }]),
      readonly,
    }))
  }
}
