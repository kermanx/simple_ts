use crate::analyzer::Analyzer;
use oxc::ast::ast::JSXSpreadChild;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_spread_child(&mut self, _node: &'a JSXSpreadChild<'a>) {}
}
