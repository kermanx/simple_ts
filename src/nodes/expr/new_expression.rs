use oxc::ast::ast::NewExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_new_expression(
    &mut self,
    node: &'a NewExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let callee = self.exec_expression(&node.callee, None);

    let callable = self.extract_callable_function(callee);

    self.exec_call(callable, &node.type_parameters, Ty::Error, &node.arguments);

    todo!()
  }
}
