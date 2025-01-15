use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::BinaryExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_binary_expression(&mut self, node: &'a BinaryExpression<'a>) -> Ty<'a> {
    let lhs = self.exec_expression(&node.left);
    let rhs = self.exec_expression(&node.right);
    self.binary_operation(node.operator, lhs, rhs)
  }
}
