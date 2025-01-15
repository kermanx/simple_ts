use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::AssignmentTargetPattern;

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_pattern_write(
    &mut self,
    node: &'a AssignmentTargetPattern<'a>,
    value: Ty<'a>,
  ) {
    match node {
      AssignmentTargetPattern::ArrayAssignmentTarget(node) => {
        let (element_values, rest_value) =
          self.destruct_as_array(value, node.elements.len(), node.rest.is_some());

        for (element, value) in node.elements.iter().zip(element_values) {
          if let Some(element) = element {
            self.exec_assignment_target_maybe_default(element, value);
          }
        }
        if let Some(rest) = &node.rest {
          self.exec_assignment_target_rest(rest, rest_value.unwrap());
        }
      }
      AssignmentTargetPattern::ObjectAssignmentTarget(node) => {
        let mut enumerated = vec![];
        for property in &node.properties {
          enumerated.push(self.exec_assignment_target_property(property, value));
        }
        if let Some(rest) = &node.rest {
          let init = self.exec_object_rest(value, enumerated);
          self.exec_assignment_target_rest(rest, init);
        }
      }
    }
  }
}
