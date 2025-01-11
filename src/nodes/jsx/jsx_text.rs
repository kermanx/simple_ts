use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::JSXText;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_text(&mut self, _node: &'a JSXText<'a>) -> Type<'a> {
    self.factory.unknown
  }
}
