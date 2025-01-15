use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::JSXFragment;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_fragment(&mut self, node: &'a JSXFragment<'a>) -> Ty<'a> {
    // already computed unknown
    self.exec_jsx_children(&node.children)
  }
}
