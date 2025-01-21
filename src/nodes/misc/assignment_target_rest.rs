use oxc::ast::ast::AssignmentTargetRest;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_rest(&mut self, node: &'a AssignmentTargetRest<'a>, value: Ty<'a>) {
    self.exec_assignment_target_write(&node.target, value, None)
  }
}
