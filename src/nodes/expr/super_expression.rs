use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::Super;

impl<'a> Analyzer<'a> {
  pub fn exec_super(&mut self, _node: &'a Super) -> Entity<'a> {
    self.factory.unknown
  }
}
