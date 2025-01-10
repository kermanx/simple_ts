use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::SequenceExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_sequence_expression(&mut self, node: &'a SequenceExpression<'a>) -> Entity<'a> {
    let mut last = None;
    for expression in &node.expressions {
      last = Some(self.exec_expression(expression));
    }
    last.unwrap()
  }
}
