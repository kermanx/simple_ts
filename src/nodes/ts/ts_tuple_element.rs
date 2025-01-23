use oxc::ast::ast::TSTupleElement;

use crate::{ty::tuple::TupleElement, Analyzer};

impl<'a> Analyzer<'a> {
  pub fn resolve_tuple_element(&mut self, node: &'a TSTupleElement<'a>) -> TupleElement<'a> {
    match node {
      TSTupleElement::TSOptionalType(node) => {
        let ty = self.resolve_type(&node.type_annotation);
        TupleElement { name: None, spread: false, optional: true, ty }
      }
      TSTupleElement::TSRestType(node) => {
        let ty = self.resolve_type(&node.type_annotation);
        TupleElement { name: None, spread: true, optional: false, ty }
      }
      TSTupleElement::TSNamedTupleMember(node) => {
        let mut el = self.resolve_tuple_element(&node.element_type);
        el.name = Some(&node.label.name);
        el.optional = node.optional;
        el
      }
      _ => {
        let ty = self.resolve_type(node.to_ts_type());
        TupleElement { name: None, spread: false, optional: false, ty }
      }
    }
  }
}
