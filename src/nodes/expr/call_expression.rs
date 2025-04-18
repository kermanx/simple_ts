use oxc::ast::ast::CallExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_call_expression(
    &mut self,
    node: &'a CallExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let (indeterminate, value) = self.exec_call_expression_in_chain(node, sat);

    if indeterminate {
      self.pop_scope();
    }

    value
  }

  pub fn exec_call_expression_in_chain(
    &mut self,
    node: &'a CallExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> (bool, Ty<'a>) {
    let (mut indeterminate, callee, this_arg) = self.exec_callee(&node.callee);

    if !indeterminate && node.optional {
      self.push_indeterminate_scope();
      indeterminate = true;
    }

    let callable = self.extract_callable_function(callee);
    let ret_val = self.exec_call(callable, &node.type_arguments, this_arg, &node.arguments, sat);

    (indeterminate, ret_val.unwrap_or(Ty::Error))
  }
}
