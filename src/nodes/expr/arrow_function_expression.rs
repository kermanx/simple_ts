use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::ArrowFunctionExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_arrow_function_expression(
    &mut self,
    node: &'a ArrowFunctionExpression<'a>,
  ) -> Ty<'a> {
    todo!()
  }
}
