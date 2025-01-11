use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::YieldExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_yield_expression(&mut self, node: &'a YieldExpression<'a>) -> Type<'a> {
    if let Some(argument) = &node.argument {
      let argument = self.exec_expression(argument);
      argument.unknown_mutation(self);
    }
    self.factory.unknown
  }
}
