use crate::analyzer::Analyzer;
use oxc::ast::ast::ReturnStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_return_statement(&mut self, node: &'a ReturnStatement) {
    let value =
      node.argument.as_ref().map_or(self.factory.undefined, |expr| self.exec_expression(expr));
    self.return_value(value);
  }
}
