use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::ArrowFunctionExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_arrow_function_expression(
    &mut self,
    node: &'a ArrowFunctionExpression<'a>,
  ) -> Type<'a> {
    todo!()
  }
}
