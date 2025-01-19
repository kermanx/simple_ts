use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::UpdateExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_update_expression(
    &mut self,
    node: &'a UpdateExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let (value, cache) = self.exec_simple_assignment_target_read(&node.argument);
    let numeric_value = self.get_to_numeric(value);
    self.exec_simple_assignment_target_write(&node.argument, numeric_value, cache);
    numeric_value
  }
}
