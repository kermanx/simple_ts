use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::CallExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_call_expression(
    &mut self,
    node: &'a CallExpression<'a>,
    ty: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let (indeterminate, value) = self.exec_call_expression_in_chain(node, ty);

    if indeterminate {
      self.pop_scope();
    }

    value
  }

  pub fn exec_call_expression_in_chain(
    &mut self,
    node: &'a CallExpression<'a>,
    ty: Option<Ty<'a>>,
  ) -> (bool, Ty<'a>) {
    let (mut indeterminate, callee, this) = self.exec_callee(&node.callee);

    if !indeterminate && node.optional {
      self.push_indeterminate_scope();
      indeterminate = true;
    }

    let callable = self.extract_callable_function(callee);
    let ret_val = self.exec_call(callable, &node.type_parameters, &node.arguments);

    (indeterminate, ret_val)
  }
}
