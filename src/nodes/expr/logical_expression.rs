use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::{LogicalExpression, LogicalOperator};

impl<'a> Analyzer<'a> {
  pub fn exec_logical_expression(&mut self, node: &'a LogicalExpression<'a>) -> Entity<'a> {
    let left = self.exec_expression(&node.left);

    let (maybe_left, maybe_right) = match &node.operator {
      LogicalOperator::And => match left.test_truthy() {
        Some(true) => (false, true),
        Some(false) => (true, false),
        None => (true, true),
      },
      LogicalOperator::Or => match left.test_truthy() {
        Some(true) => (true, false),
        Some(false) => (false, true),
        None => (true, true),
      },
      LogicalOperator::Coalesce => match left.test_nullish() {
        Some(true) => (false, true),
        Some(false) => (true, false),
        None => (true, true),
      },
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
        self.factory.logical_result(left, right, node.operator)
      }
      (false, false) => unreachable!("Logical expression should have at least one possible branch"),
    };

    value
  }
}
