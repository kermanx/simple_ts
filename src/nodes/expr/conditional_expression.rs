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
    let (blocked_1, shadow_1) = self.pop_scope_subtle();

    self.push_exit_blocker_scope();
    let alternate = self.exec_expression(&node.alternate);
    let (blocked_2, shadow_2) = self.pop_scope_subtle();

    self.apply_complementary_blocked_exits(blocked_1, blocked_2);
    self.apply_complementary_shadows([shadow_1, shadow_2]);

    into_union(self.allocator, [consequent, alternate])
  }
}
