use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::Super;

impl<'a> Analyzer<'a> {
  pub fn exec_super(&mut self, _node: &'a Super) -> Type<'a> {
    self.factory.unknown
  }
}
