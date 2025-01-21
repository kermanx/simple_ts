use crate::{ty::Ty, Analyzer};
use oxc::ast::ast::TSNonNullExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_ts_non_null_expression(
    &mut self,
    node: &'a TSNonNullExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let ty = self.exec_expression(&node.expression, sat);
    self.non_nullable(ty)
  }
}
