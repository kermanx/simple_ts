use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::BinaryExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_binary_expression(
    &mut self,
    node: &'a BinaryExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let lhs = self.exec_expression(&node.left, None);
    let rhs = self.exec_expression(&node.right, None);
    self.binary_operation(node.operator, lhs, rhs)
  }
}
