use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::AssignmentTargetRest;

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_rest(
    &mut self,
    node: &'a AssignmentTargetRest<'a>,
    value: Type<'a>,
  ) {
    self.exec_assignment_target_write(&node.target, value, None)
  }
}
