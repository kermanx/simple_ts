use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::ast::ast::ConditionalExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_conditional_expression(&mut self, node: &'a ConditionalExpression<'a>) -> Ty<'a> {
    self.exec_expression(&node.test);

    self.push_exit_blocker_scope();
    let consequent = self.exec_expression(&node.consequent);
    let scope_1 = self.scopes.pop();

    self.push_exit_blocker_scope();
    let alternate = self.exec_expression(&node.alternate);
    let scope_2 = self.scopes.pop();

    self.finalize_complementary_scopes(scope_1, scope_2);

    into_union(self.allocator, [consequent, alternate])
  }
}
