use crate::{
  analyzer::Analyzer,
  r#type::{union::into_union, Type},
};
use oxc::ast::ast::LogicalExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_logical_expression(&mut self, node: &'a LogicalExpression<'a>) -> Type<'a> {
    let left = self.exec_expression(&node.left);

    self.push_indeterminate_cf_scope();
    let right = self.exec_expression(&node.right);
    self.pop_cf_scope();

    into_union(self.allocator, vec![left, right])
  }
}
