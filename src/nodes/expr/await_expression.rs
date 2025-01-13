use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::AwaitExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_await_expression(&mut self, node: &'a AwaitExpression<'a>) -> Type<'a> {
    let call_scope = self.call_scope_mut();
    if !call_scope.is_async {
      self.add_diagnostic("SyntaxError: await is only valid in async functions");
    }

    let value = self.exec_expression(&node.argument);
    self.get_to_awaited(value)
  }
}
