use oxc::ast::ast::ThisExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_this_expression(
    &mut self,
    _node: &'a ThisExpression,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    self.call_scopes.last().unwrap().this
  }
}
