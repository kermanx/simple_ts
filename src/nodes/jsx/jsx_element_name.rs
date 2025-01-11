use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::JSXElementName;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_element_name(&mut self, node: &'a JSXElementName<'a>) -> Entity<'a> {
    match node {
      JSXElementName::Identifier(_node) => self.factory.string,
      JSXElementName::IdentifierReference(node) => self.exec_identifier_reference_read(node),
      JSXElementName::NamespacedName(_node) => self.factory.string,
      JSXElementName::MemberExpression(node) => self.exec_jsx_member_expression(node),
      JSXElementName::ThisExpression(node) => self.exec_this_expression(node),
    }
  }
}
