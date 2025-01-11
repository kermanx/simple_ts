use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::JSXFragment;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_fragment(&mut self, node: &'a JSXFragment<'a>) -> Type<'a> {
    // already computed unknown
    self.exec_jsx_children(&node.children)
  }
}
