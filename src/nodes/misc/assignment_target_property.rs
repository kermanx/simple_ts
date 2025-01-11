use crate::{analyzer::Analyzer, ast::AstKind2, entity::Entity};
use oxc::ast::ast::AssignmentTargetProperty;

#[derive(Debug, Default)]
struct IdentifierData {
  need_init: bool,
}

impl<'a> Analyzer<'a> {
  /// Returns the key
  pub fn exec_assignment_target_property(
    &mut self,
    node: &'a AssignmentTargetProperty<'a>,
    value: Entity<'a>,
  ) -> Entity<'a> {
    match node {
      AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(node) => {
        let key = self.factory.string_literal(node.binding.name.as_str());

        let value = value.get_property(self, key);

        let (need_init, value) = if let Some(init) = &node.init {
          self.exec_with_default(init, value)
        } else {
          (false, value)
        };

        let data =
          self.load_data::<IdentifierData>(AstKind2::AssignmentTargetPropertyIdentifier(node));
        data.need_init |= need_init;

        self.exec_identifier_reference_write(&node.binding, value);

        key
      }
      AssignmentTargetProperty::AssignmentTargetPropertyProperty(node) => {
        let key = self.exec_property_key(&node.name);

        let value = value.get_property(self, key);
        self.exec_assignment_target_maybe_default(&node.binding, value);
        key
      }
    }
  }
}
