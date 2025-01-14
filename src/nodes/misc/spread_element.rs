use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::SpreadElement;

impl<'a> Analyzer<'a> {
  pub fn exec_spread_element(&mut self, node: &'a SpreadElement<'a>) -> Type<'a> {
    let argument = self.exec_expression(&node.argument);
    self.iterate_result_union(argument)
  }
}
