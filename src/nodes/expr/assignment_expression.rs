use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::{AssignmentExpression, AssignmentOperator, BinaryOperator, LogicalOperator};

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_expression(&mut self, node: &'a AssignmentExpression<'a>) -> Type<'a> {
    if node.operator == AssignmentOperator::Assign {
      let rhs = self.exec_expression(&node.right);
      self.exec_assignment_target_write(&node.left, rhs, None);
      rhs
    } else if node.operator.is_logical() {
      let (left, cache) = self.exec_assignment_target_read(&node.left);

      let (maybe_left, maybe_right) = match &node.operator {
        AssignmentOperator::LogicalAnd => match left.test_truthy() {
          Some(true) => (false, true),
          Some(false) => (true, false),
          None => (true, true),
        },
        AssignmentOperator::LogicalOr => match left.test_truthy() {
          Some(true) => (true, false),
          Some(false) => (false, true),
          None => (true, true),
        },
        AssignmentOperator::LogicalNullish => match left.test_nullish() {
          Some(true) => (false, true),
          Some(false) => (true, false),
          None => (true, true),
        },
        _ => unreachable!(),
      };

      let exec_right = |analyzer: &mut Analyzer<'a>| {
        analyzer.exec_optional_indeterminately(maybe_left && maybe_right, |analyzer| {
          analyzer.exec_expression(&node.right)
        })
      };

      let value = match (maybe_left, maybe_right) {
        (false, true) => exec_right(self),
        (true, false) => left,
        (true, true) => {
          let right = exec_right(self);
          self.factory.logical_result(left, right, to_logical_operator(node.operator))
        }
        (false, false) => {
          unreachable!("Logical assignment expression should have at least one side")
        }
      };

      if maybe_right {
        self.exec_assignment_target_write(&node.left, value, cache);
      }

      value
    } else {
      let (lhs, cache) = self.exec_assignment_target_read(&node.left);
      let rhs = self.exec_expression(&node.right);
      let value = self.entity_op.binary_op(self, to_binary_operator(node.operator), lhs, rhs);
      self.exec_assignment_target_write(&node.left, value, cache);
      value
    }
  }
}

fn to_logical_operator(operator: AssignmentOperator) -> LogicalOperator {
  match operator {
    AssignmentOperator::LogicalAnd => LogicalOperator::And,
    AssignmentOperator::LogicalOr => LogicalOperator::Or,
    AssignmentOperator::LogicalNullish => LogicalOperator::Coalesce,
    _ => unreachable!(),
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
