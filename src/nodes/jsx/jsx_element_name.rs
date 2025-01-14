use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::JSXElementName;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_element_name(&mut self, node: &'a JSXElementName<'a>) -> Type<'a> {
    match node {
      JSXElementName::Identifier(_node) => todo!("Jsx intrinsic element"),
      JSXElementName::IdentifierReference(node) => self.exec_identifier_reference_read(node),
      JSXElementName::NamespacedName(_node) => todo!("Jsx intrinsic element"),
      JSXElementName::MemberExpression(node) => self.exec_jsx_member_expression(node),
      JSXElementName::ThisExpression(node) => self.exec_this_expression(node),
    }
  }
}
