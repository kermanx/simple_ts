use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::PropertyKey;

impl<'a> Analyzer<'a> {
  pub fn exec_property_key(&mut self, node: &'a PropertyKey<'a>) -> Ty<'a> {
    match node {
      PropertyKey::StaticIdentifier(node) => self.exec_identifier_name(node),
      PropertyKey::PrivateIdentifier(node) => self.exec_private_identifier(node),
      node => self.exec_expression(node.to_expression())
    }
  }
}
