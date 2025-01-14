use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::ThisExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_this_expression(&mut self, _node: &'a ThisExpression) -> Type<'a> {
    self.call_scopes.last().unwrap().this
  }
}
