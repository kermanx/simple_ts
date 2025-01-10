use crate::{analyzer::Analyzer, ast::Arguments, entity::Entity};
use oxc::ast::ast::Argument;

impl<'a> Analyzer<'a> {
  pub fn exec_arguments(&mut self, node: &'a Arguments<'a>) -> Entity<'a> {
    let mut arguments = vec![];
    for argument in node {
      let (spread, val) = match argument {
        Argument::SpreadElement(node) => (true, self.exec_expression(&node.argument)),
        node => (false, self.exec_expression(node.to_expression())),
      };
      arguments.push((spread, val));
    }
    self.factory.arguments(arguments)
  }
}
