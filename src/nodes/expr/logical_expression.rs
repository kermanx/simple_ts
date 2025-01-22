use oxc::ast::ast::LogicalExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_logical_expression(
    &mut self,
    node: &'a LogicalExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let left = self.exec_expression(&node.left, sat);

    self.push_indeterminate_scope();
    let right = self.exec_expression(&node.right, sat);
    self.pop_scope();

    self.into_union([left, right])
  }
}
