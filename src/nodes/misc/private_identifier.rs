use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::PrivateIdentifier;

impl<'a> Analyzer<'a> {
  pub fn exec_private_identifier(&mut self, node: &'a PrivateIdentifier<'a>) -> Entity<'a> {
    // FIXME: Not good
    self.factory.string_literal(self.escape_private_identifier_name(node.name.as_str()))
  }
}
