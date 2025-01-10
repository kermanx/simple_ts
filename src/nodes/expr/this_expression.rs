use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::ThisExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_this_expression(&mut self, _node: &'a ThisExpression) -> Entity<'a> {
    self.get_this()
  }
}
