use crate::analyzer::Analyzer;
use oxc::ast::ast::{FunctionBody, Statement};

impl<'a> Analyzer<'a> {
  pub fn exec_function_body(&mut self, node: &'a FunctionBody<'a>) {
    self.exec_statement_vec(&node.statements);
  }

  pub fn exec_function_expression_body(&mut self, node: &'a FunctionBody<'a>) {
    if let [Statement::ExpressionStatement(expr)] = node.statements.as_slice() {
      let value = self.exec_expression(&expr.expression);
      self.add_returned_value(value);
    } else {
      unreachable!();
    }
  }
}
