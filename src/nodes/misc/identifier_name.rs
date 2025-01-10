use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::IdentifierName;

impl<'a> Analyzer<'a> {
  pub fn exec_identifier_name(&mut self, node: &'a IdentifierName<'a>) -> Entity<'a> {
    self.factory.string(node.name.as_str())
  }
}
