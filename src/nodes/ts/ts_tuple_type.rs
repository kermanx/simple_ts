use oxc::ast::ast::TSTupleType;

use crate::{
  ty::{tuple::TupleType, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn resolve_tuple_type(&mut self, node: &'a TSTupleType<'a>) -> Ty<'a> {
    Ty::Tuple(self.allocator.alloc(TupleType {
      elements: node.element_types.iter().map(|el| self.resolve_tuple_element(el)).collect(),
      readonly: false,
    }))
  }
}
