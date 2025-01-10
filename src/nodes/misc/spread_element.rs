use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::SpreadElement;

impl<'a> Analyzer<'a> {
  pub fn exec_spread_element(&mut self, node: &'a SpreadElement<'a>) -> Option<Entity<'a>> {
    let argument = self.exec_expression(&node.argument);
    argument.iterate_result_union(self)
  }
}
