use oxc::ast::ast::ParenthesizedExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_parenthesized_expression(
    &mut self,
    node: &'a ParenthesizedExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    self.exec_expression(&node.expression, sat)
  }
}
