use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::NewExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_new_expression(&mut self, node: &'a NewExpression<'a>) -> Ty<'a> {
    let callee = self.exec_expression(&node.callee);

    let arguments = self.exec_arguments(&node.arguments);

    let value = self.get_instantiation_return(callee, arguments);

    value
  }
}
