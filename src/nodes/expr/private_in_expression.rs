use oxc::ast::ast::PrivateInExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_private_in_expression(
    &mut self,
    node: &'a PrivateInExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    self.exec_expression(&node.right, None);
    Ty::Boolean
  }
}
