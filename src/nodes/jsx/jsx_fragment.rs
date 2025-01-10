use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::JSXFragment;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_fragment(&mut self, node: &'a JSXFragment<'a>) -> Entity<'a> {
    // already computed unknown
    self.exec_jsx_children(&node.children)
  }
}
