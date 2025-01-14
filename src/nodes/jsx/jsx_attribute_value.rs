use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::JSXAttributeValue;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attribute_value(&mut self, node: &'a Option<JSXAttributeValue<'a>>) -> Type<'a> {
    if let Some(node) = node {
      match node {
        JSXAttributeValue::StringLiteral(node) => self.exec_string_literal(node),
        JSXAttributeValue::ExpressionContainer(node) => {
          self.exec_jsx_expression_container_as_attribute_value(node)
        }
        JSXAttributeValue::Element(node) => self.exec_jsx_element(node),
        JSXAttributeValue::Fragment(node) => self.exec_jsx_fragment(node),
      }
    } else {
      Type::Boolean
    }
  }
}
