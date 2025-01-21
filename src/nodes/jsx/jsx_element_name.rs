use oxc::ast::ast::JSXElementName;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_element_name(&mut self, node: &'a JSXElementName<'a>) -> Ty<'a> {
    match node {
      JSXElementName::Identifier(_node) => todo!("Jsx intrinsic element"),
      JSXElementName::IdentifierReference(node) => self.exec_identifier_reference_read(node, None),
      JSXElementName::NamespacedName(_node) => todo!("Jsx intrinsic element"),
      JSXElementName::MemberExpression(node) => self.exec_jsx_member_expression(node, None),
      JSXElementName::ThisExpression(node) => self.exec_this_expression(node, None),
    }
  }
}
