use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::ImportExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_import_expression(&mut self, node: &'a ImportExpression<'a>) -> Type<'a> {
    let source = self.exec_expression(&node.source);

    for argument in &node.arguments {
      self.exec_expression(argument);
    }

    todo!()
  }
}
