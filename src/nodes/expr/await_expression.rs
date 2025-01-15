use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::AwaitExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_await_expression(&mut self, node: &'a AwaitExpression<'a>) -> Ty<'a> {
    let value = self.exec_expression(&node.argument);
    self.get_to_awaited(value)
  }
}
