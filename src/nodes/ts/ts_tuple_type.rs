use oxc::ast::ast::TSTupleType;

use crate::{
  Analyzer,
  ty::{Ty, tuple::TupleType},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_tuple_type(&mut self, node: &'a TSTupleType<'a>, readonly: bool) -> Ty<'a> {
    Ty::Tuple(
      self.allocator.alloc(TupleType {
        elements: self
          .allocator
          .alloc_slice(node.element_types.iter().map(|el| self.resolve_tuple_element(el))),
        readonly,
      }),
    )
  }
}
