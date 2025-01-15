use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::ast::ast::LogicalExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_logical_expression(&mut self, node: &'a LogicalExpression<'a>) -> Ty<'a> {
    let left = self.exec_expression(&node.left);

    self.push_indeterminate_cf_scope();
    let right = self.exec_expression(&node.right);
    self.pop_cf_scope();

    into_union(self.allocator, [left, right])
  }
}
