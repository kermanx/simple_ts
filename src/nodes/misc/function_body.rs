use crate::analyzer::Analyzer;
use oxc::ast::ast::{FunctionBody, Statement};

impl<'a> Analyzer<'a> {
  pub fn exec_function_body(&mut self, node: &'a FunctionBody<'a>) {
    self.exec_statement_vec(&node.statements);
  }

  pub fn exec_function_expression_body(&mut self, node: &'a FunctionBody<'a>) {
    assert!(node.statements.len() == 1);
    if let Some(Statement::ExpressionStatement(expr)) = node.statements.first() {
      let value = self.exec_expression(&expr.expression);
      let call_scope = self.call_scope_mut();
      call_scope.returned_values.push(value);
    } else {
      unreachable!();
    }
  }
}
