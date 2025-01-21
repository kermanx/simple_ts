use oxc::ast::ast::TSTypeAssertion;

use crate::{ty::Ty, Analyzer};

impl<'a> Analyzer<'a> {
  pub fn exec_ts_type_assertion(
    &mut self,
    node: &'a TSTypeAssertion<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let ty = self.resolve_type(&node.type_annotation);
    self.exec_expression(&node.expression, Some(ty));
    ty
  }
}
