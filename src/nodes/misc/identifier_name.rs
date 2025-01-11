use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::IdentifierName;

impl<'a> Analyzer<'a> {
  pub fn exec_identifier_name(&mut self, node: &'a IdentifierName<'a>) -> Type<'a> {
    self.factory.string_literal(node.name.as_str())
  }
}
