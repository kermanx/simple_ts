use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::ParenthesizedExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_parenthesized_expression(&mut self, node: &'a ParenthesizedExpression<'a>) -> Ty<'a> {
    self.exec_expression(&node.expression)
  }
}
