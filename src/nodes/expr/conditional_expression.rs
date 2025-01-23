use oxc::ast::ast::ConditionalExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_conditional_expression(
    &mut self,
    node: &'a ConditionalExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    self.exec_expression(&node.test, None);

    self.push_exit_blocker_scope();
    let consequent = self.exec_expression(&node.consequent, sat);
    let scope_1 = self.scopes.pop();

    self.push_exit_blocker_scope();
    let alternate = self.exec_expression(&node.alternate, sat);
    let scope_2 = self.scopes.pop();

    self.finalize_complementary_scopes(scope_1, scope_2);

    self.into_union([consequent, alternate]).unwrap()
  }
}
