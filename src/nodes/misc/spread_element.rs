use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::SpreadElement;

impl<'a> Analyzer<'a> {
  pub fn exec_spread_element(&mut self, node: &'a SpreadElement<'a>) -> Ty<'a> {
    let argument = self.exec_expression(&node.argument);
    self.iterate_result_union(argument)
  }
}
