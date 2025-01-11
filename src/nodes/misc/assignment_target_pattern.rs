use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::AssignmentTargetPattern;

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_pattern_write(
    &mut self,
    node: &'a AssignmentTargetPattern<'a>,
    value: Entity<'a>,
  ) {
    match node {
      AssignmentTargetPattern::ArrayAssignmentTarget(node) => {
        let (element_values, rest_value) =
          value.destruct_as_array(self, node.elements.len(), node.rest.is_some());

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
        let is_nullish = value.test_nullish();
        if is_nullish != Some(false) {
          if is_nullish == Some(true) {
            self.thrown_builtin_error("Cannot destructure nullish value");
          } else {
            self.may_throw();
          }
          value.unknown_mutation(self);
        }

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
