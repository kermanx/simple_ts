use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::YieldExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_yield_expression(&mut self, node: &'a YieldExpression<'a>) -> Entity<'a> {
    if let Some(argument) = &node.argument {
      let argument = self.exec_expression(argument);
      argument.consume(self);
    }
    self.factory.unknown
  }
}
