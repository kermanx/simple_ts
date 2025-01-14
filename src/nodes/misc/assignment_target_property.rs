use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::AssignmentTargetProperty;

impl<'a> Analyzer<'a> {
  /// Returns the key
  pub fn exec_assignment_target_property(
    &mut self,
    node: &'a AssignmentTargetProperty<'a>,
    value: Type<'a>,
  ) -> Type<'a> {
    match node {
      AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(node) => {
        let key = Type::StringLiteral(&node.binding.name);

        let value = self.get_property(value, key);

        let value =
          if let Some(init) = &node.init { self.exec_with_default(init, value) } else { value };

        self.exec_identifier_reference_write(&node.binding, value);

        key
      }
      AssignmentTargetProperty::AssignmentTargetPropertyProperty(node) => {
        let key = self.exec_property_key(&node.name);

        let value = self.get_property(value, key);

        self.exec_assignment_target_maybe_default(&node.binding, value);

        key
      }
    }
  }
}
