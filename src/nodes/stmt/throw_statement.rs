use crate::analyzer::Analyzer;
use oxc::ast::ast::ThrowStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_throw_statement(&mut self, node: &'a ThrowStatement<'a>) {
    let value = self.exec_expression(&node.argument);

    self.explicit_throw(value);
  }
}
