use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::JSXAttributeValue;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attribute_value(
    &mut self,
    node: &'a Option<JSXAttributeValue<'a>>,
    ty: Option<Ty<'a>>,
  ) -> Ty<'a> {
    if let Some(node) = node {
      match node {
        JSXAttributeValue::StringLiteral(node) => self.exec_string_literal(node, ty),
        JSXAttributeValue::ExpressionContainer(node) => {
          self.exec_jsx_expression_container_as_attribute_value(node, ty)
        }
        JSXAttributeValue::Element(node) => self.exec_jsx_element(node, ty),
        JSXAttributeValue::Fragment(node) => self.exec_jsx_fragment(node, ty),
      }
    } else {
      Ty::Boolean
    }
  }
}
