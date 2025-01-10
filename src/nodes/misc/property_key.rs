use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::PropertyKey;

impl<'a> Analyzer<'a> {
  pub fn exec_property_key(&mut self, node: &'a PropertyKey<'a>) -> Entity<'a> {
    match node {
      PropertyKey::StaticIdentifier(node) => self.exec_identifier_name(node),
      PropertyKey::PrivateIdentifier(node) => self.exec_private_identifier(node),
      node => self.exec_expression(node.to_expression()).get_to_property_key(self),
    }
  }
}
