use crate::{ty::Ty, Analyzer};
use oxc::ast::ast::TSSatisfiesExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_ts_satisfies_expression(
    &mut self,
    node: &'a TSSatisfiesExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let ty = self.resolve_type_or_defer(&node.type_annotation);

    self.exec_expression(&node.expression, Some(ty))
  }
}
