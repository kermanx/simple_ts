use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::JSXText;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_text(&mut self, _node: &'a JSXText<'a>) {
    // Do nothing
  }
}
