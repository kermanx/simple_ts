use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::AwaitExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_await_expression(&mut self, node: &'a AwaitExpression<'a>) -> Type<'a> {
    let value = self.exec_expression(&node.argument);
    self.get_to_awaited(value)
  }
}
