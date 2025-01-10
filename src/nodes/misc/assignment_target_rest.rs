use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::AssignmentTargetRest;

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_rest(
    &mut self,
    node: &'a AssignmentTargetRest<'a>,
    value: Entity<'a>,
  ) {
    self.exec_assignment_target_write(&node.target, value, None)
  }
}
