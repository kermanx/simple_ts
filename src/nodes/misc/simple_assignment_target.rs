use crate::{
  analyzer::Analyzer,
  ty::{property_key::PropertyKeyType, Ty},
};
use oxc::ast::{ast::SimpleAssignmentTarget, match_member_expression};

impl<'a> Analyzer<'a> {
  pub fn exec_simple_assignment_target_read(
    &mut self,
    node: &'a SimpleAssignmentTarget<'a>,
  ) -> (Ty<'a>, Option<(Ty<'a>, PropertyKeyType<'a>)>) {
    match node {
      match_member_expression!(SimpleAssignmentTarget) => {
        let (value, cache) = self.exec_member_expression_read(node.to_member_expression());
        (value, Some(cache))
      }
      SimpleAssignmentTarget::AssignmentTargetIdentifier(node) => {
        (self.exec_identifier_reference_read(node), None)
      }
      _ => unreachable!(),
    }
  }

  pub fn exec_simple_assignment_target_write(
    &mut self,
    node: &'a SimpleAssignmentTarget<'a>,
    value: Ty<'a>,
    cache: Option<(Ty<'a>, PropertyKeyType<'a>)>,
  ) {
    match node {
      match_member_expression!(SimpleAssignmentTarget) => {
        self.exec_member_expression_write(node.to_member_expression(), value, cache)
      }
      SimpleAssignmentTarget::AssignmentTargetIdentifier(node) => {
        self.exec_identifier_reference_write(node, value)
      }
      _ => unreachable!(),
    }
  }
}
