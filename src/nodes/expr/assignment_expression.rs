use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::ast::ast::{AssignmentExpression, AssignmentOperator, BinaryOperator};

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_expression(
    &mut self,
    node: &'a AssignmentExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    if node.operator == AssignmentOperator::Assign {
      let (left, cache) = self.exec_assignment_target_read(&node.left);
      let rhs = self.exec_expression(&node.right, Some(left));
      self.exec_assignment_target_write(&node.left, rhs, cache);
      rhs
    } else if node.operator.is_logical() {
      let (left, cache) = self.exec_assignment_target_read(&node.left);

      self.push_indeterminate_scope();
      let right = self.exec_expression(&node.right, Some(left));
      let value = into_union(self.allocator, [left, right]);
      self.pop_scope();

      // Execute write outside of the indeterminate scope, because the value is already an union
      self.exec_assignment_target_write(&node.left, value, cache);

      value
    } else {
      let (left, cache) = self.exec_assignment_target_read(&node.left);
      let right = self.exec_expression(&node.right, Some(left));
      let value = self.binary_operation(to_binary_operator(node.operator), left, right);
      self.exec_assignment_target_write(&node.left, value, cache);
      value
    }
  }
}

fn to_binary_operator(operator: AssignmentOperator) -> BinaryOperator {
  match operator {
    AssignmentOperator::Addition => BinaryOperator::Addition,
    AssignmentOperator::Subtraction => BinaryOperator::Subtraction,
    AssignmentOperator::Multiplication => BinaryOperator::Multiplication,
    AssignmentOperator::Division => BinaryOperator::Division,
    AssignmentOperator::Remainder => BinaryOperator::Remainder,
    AssignmentOperator::Exponential => BinaryOperator::Exponential,
    AssignmentOperator::BitwiseAnd => BinaryOperator::BitwiseAnd,
    AssignmentOperator::BitwiseOR => BinaryOperator::BitwiseOR,
    AssignmentOperator::BitwiseXOR => BinaryOperator::BitwiseXOR,
    AssignmentOperator::ShiftLeft => BinaryOperator::ShiftLeft,
    AssignmentOperator::ShiftRight => BinaryOperator::ShiftRight,
    AssignmentOperator::ShiftRightZeroFill => BinaryOperator::ShiftRightZeroFill,
    _ => unreachable!(),
  }
}
