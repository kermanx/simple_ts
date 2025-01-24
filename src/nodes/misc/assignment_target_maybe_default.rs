use oxc::ast::ast::AssignmentTargetMaybeDefault;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_maybe_default(
    &mut self,
    node: &'a AssignmentTargetMaybeDefault<'a>,
    value: Ty<'a>,
  ) {
    match node {
      AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(node) => {
        let value = self.exec_with_default(&node.init, Some(value));

        self.exec_assignment_target_write(&node.binding, value, None);
      }
      _ => self.exec_assignment_target_write(node.to_assignment_target(), value, None),
    }
  }
}
