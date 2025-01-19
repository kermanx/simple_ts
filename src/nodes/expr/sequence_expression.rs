use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::SequenceExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_sequence_expression(
    &mut self,
    node: &'a SequenceExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let len = node.expressions.len();
    for expression in &node.expressions[..len - 1] {
      self.exec_expression(expression, None);
    }
    self.exec_expression(&node.expressions[len - 1], sat)
  }
}
