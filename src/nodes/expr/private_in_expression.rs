use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::PrivateInExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_private_in_expression(&mut self, node: &'a PrivateInExpression<'a>) -> Ty<'a> {
    self.exec_expression(&node.right);
    Ty::Boolean
  }
}
