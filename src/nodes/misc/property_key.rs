use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::PropertyKey;

impl<'a> Analyzer<'a> {
  pub fn exec_property_key(&mut self, node: &'a PropertyKey<'a>) -> Type<'a> {
    match node {
      PropertyKey::StaticIdentifier(node) => self.exec_identifier_name(node),
      PropertyKey::PrivateIdentifier(node) => self.exec_private_identifier(node),
      node => {
        let value = self.exec_expression(node.to_expression());
        self.get_to_property_key(value)
      }
    }
  }
}
