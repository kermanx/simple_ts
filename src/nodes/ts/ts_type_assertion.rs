use oxc::ast::ast::TSTypeAssertion;

use crate::{Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_ts_type_assertion(
    &mut self,
    node: &'a TSTypeAssertion<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    if node.type_annotation.is_const_type_reference() {
      self.exec_expression_with_as_const(&node.expression, sat, true)
    } else {
      let ty = self.resolve_type(&node.type_annotation);
      self.exec_expression(&node.expression, Some(ty));
      ty
    }
  }
}
