use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::YieldExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_yield_expression(&mut self, node: &'a YieldExpression<'a>) -> Type<'a> {
    todo!()
  }
}
