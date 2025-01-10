use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::ParenthesizedExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_parenthesized_expression(
    &mut self,
    node: &'a ParenthesizedExpression<'a>,
  ) -> Entity<'a> {
    self.exec_expression(&node.expression)
  }
}
