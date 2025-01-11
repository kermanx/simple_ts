use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::PrivateInExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_private_in_expression(&mut self, node: &'a PrivateInExpression<'a>) -> Type<'a> {
    self.exec_expression(&node.right);
    self.factory.boolean
  }
}
