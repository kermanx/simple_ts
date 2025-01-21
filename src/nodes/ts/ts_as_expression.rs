use oxc::ast::ast::TSAsExpression;

use crate::{ty::Ty, Analyzer};

impl<'a> Analyzer<'a> {
  pub fn exec_ts_as_expression(
    &mut self,
    node: &'a TSAsExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let ty = self.resolve_type(&node.type_annotation);

    self.exec_expression(&node.expression, Some(ty));

    ty
  }
}
