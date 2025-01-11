use crate::{
  analyzer::Analyzer,
  entity::{Entity, EntityTrait},
};
use oxc::ast::ast::{ObjectExpression, ObjectPropertyKind, PropertyKey};

impl<'a> Analyzer<'a> {
  pub fn exec_object_expression(&mut self, node: &'a ObjectExpression) -> Entity<'a> {
    let object = self.new_empty_object(&self.builtins.prototypes.object);

    let mut has_proto = false;

    for property in &node.properties {
      match property {
        ObjectPropertyKind::ObjectProperty(node) => {
          let key = self.exec_property_key(&node.key);
          let value = self.exec_expression(&node.value);
          let value = value;

          if matches!(&node.key, PropertyKey::StaticIdentifier(node) if node.name == "__proto__") {
            has_proto = true;
          } else {
            object.init_property(self, node.kind, key, value, true);
          }
        }
        ObjectPropertyKind::SpreadProperty(node) => {
          let argument = self.exec_expression(&node.argument);
          object.init_spread(self, argument);
        }
      }
    }

    if has_proto {
      // Deoptimize the object
      object.unknown_mutation(self);
    }

    object
  }
}
