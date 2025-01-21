use oxc::ast::ast::JSXText;

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_text(&mut self, _node: &'a JSXText<'a>) {
    // Do nothing
  }
}
