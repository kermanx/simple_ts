use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::NewExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_new_expression(&mut self, node: &'a NewExpression<'a>) -> Type<'a> {
    let callee = self.exec_expression(&node.callee);

    let arguments = self.exec_arguments(&node.arguments);

    let value = callee.construct(self, arguments);

    value
  }
}
