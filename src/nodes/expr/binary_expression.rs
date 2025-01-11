use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::BinaryExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_binary_expression(&mut self, node: &'a BinaryExpression<'a>) -> Type<'a> {
    let lhs = self.exec_expression(&node.left);
    let rhs = self.exec_expression(&node.right);

    self.entity_op.binary_op(self, node.operator, lhs, rhs)
  }
}
