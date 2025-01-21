use oxc::ast::ast::ThrowStatement;

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_throw_statement(&mut self, node: &'a ThrowStatement<'a>) {
    self.exec_expression(&node.argument, None);
  }
}
