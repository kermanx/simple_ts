use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::ast::ast::ConditionalExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_conditional_expression(&mut self, node: &'a ConditionalExpression<'a>) -> Ty<'a> {
    self.exec_expression(&node.test);

    self.push_indeterminate_cf_scope();

    self.push_variable_scope();
    let consequent = self.exec_expression(&node.consequent);
    let shadow_1 = self.pop_variable_scope_no_apply_shadow();

    self.push_variable_scope();
    let alternate = self.exec_expression(&node.alternate);
    let shadow_2 = self.pop_variable_scope_no_apply_shadow();

    self.apply_complementary_shadows([shadow_1, shadow_2]);
    self.pop_cf_scope();

    into_union(self.allocator, [consequent, alternate])
  }
}
