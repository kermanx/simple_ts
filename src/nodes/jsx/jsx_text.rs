use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::JSXText;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_text(&mut self, _node: &'a JSXText<'a>) -> Entity<'a> {
    self.factory.unknown
  }
}
