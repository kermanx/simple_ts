use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::CallExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_call_expression(&mut self, node: &'a CallExpression) -> Ty<'a> {
    let (indeterminate, value) = self.exec_call_expression_in_chain(node);

    if indeterminate {
      self.pop_scope();
    }

    value
  }

  pub fn exec_call_expression_in_chain(&mut self, node: &'a CallExpression) -> (bool, Ty<'a>) {
    let (mut indeterminate, callee, this) = self.exec_callee(&node.callee);

    if !indeterminate && node.optional {
      self.push_indeterminate_scope();
      indeterminate = true;
    }

    let args = self.exec_arguments(&node.arguments);

    let ret_val = self.get_call_return(callee, this, args);

    (indeterminate, ret_val)
  }
}
