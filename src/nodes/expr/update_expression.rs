use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::UpdateExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_update_expression(&mut self, node: &'a UpdateExpression<'a>) -> Type<'a> {
    let (value, cache) = self.exec_simple_assignment_target_read(&node.argument);
    let numeric_value = value.get_to_numeric(self);
    let updated_value = self.entity_op.update(self, numeric_value, node.operator);
    self.exec_simple_assignment_target_write(&node.argument, updated_value, cache);
    if node.prefix {
      updated_value
    } else {
      numeric_value
    }
  }
}
