use oxc::ast::ast::ArrowFunctionExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_arrow_function_expression(
    &mut self,
    node: &'a ArrowFunctionExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    todo!()
  }
}
