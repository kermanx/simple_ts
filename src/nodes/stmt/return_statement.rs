use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::ReturnStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_return_statement(&mut self, node: &'a ReturnStatement) {
    let argument = if let Some(argument) = &node.argument {
      self.exec_expression(argument)
    } else {
      Ty::Undefined
    };
    self.add_returned_value(argument);
  }
}
