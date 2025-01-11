use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::ParenthesizedExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_parenthesized_expression(
    &mut self,
    node: &'a ParenthesizedExpression<'a>,
  ) -> Type<'a> {
    self.exec_expression(&node.expression)
  }
}
