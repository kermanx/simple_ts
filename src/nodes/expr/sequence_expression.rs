use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::SequenceExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_sequence_expression(&mut self, node: &'a SequenceExpression<'a>) -> Type<'a> {
    let mut last = None;
    for expression in &node.expressions {
      last = Some(self.exec_expression(expression));
    }
    last.unwrap()
  }
}
