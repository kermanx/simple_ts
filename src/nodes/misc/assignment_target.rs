use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::{
  ast::AssignmentTarget, match_assignment_target_pattern, match_simple_assignment_target,
};

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_read(
    &mut self,
    node: &'a AssignmentTarget<'a>,
  ) -> (Entity<'a>, Option<(Entity<'a>, Entity<'a>)>) {
    match node {
      match_simple_assignment_target!(AssignmentTarget) => {
        self.exec_simple_assignment_target_read(node.to_simple_assignment_target())
      }
      match_assignment_target_pattern!(AssignmentTarget) => {
        unreachable!()
      }
    }
  }

  pub fn exec_assignment_target_write(
    &mut self,
    node: &'a AssignmentTarget<'a>,
    value: Entity<'a>,
    cache: Option<(Entity<'a>, Entity<'a>)>,
  ) {
    match node {
      match_simple_assignment_target!(AssignmentTarget) => {
        self.exec_simple_assignment_target_write(node.to_simple_assignment_target(), value, cache);
      }
      match_assignment_target_pattern!(AssignmentTarget) => {
        self.exec_assignment_target_pattern_write(node.to_assignment_target_pattern(), value);
      }
    }
  }
}
