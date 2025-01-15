use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::ImportExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_import_expression(&mut self, node: &'a ImportExpression<'a>) -> Ty<'a> {
    let source = self.exec_expression(&node.source);

    for argument in &node.arguments {
      self.exec_expression(argument);
    }

    todo!()
  }
}
