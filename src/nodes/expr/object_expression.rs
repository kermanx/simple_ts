use crate::{
  analyzer::Analyzer,
  ty::{record::Record, Ty},
};
use oxc::ast::ast::{ObjectExpression, ObjectPropertyKind, PropertyKind};

impl<'a> Analyzer<'a> {
  pub fn exec_object_expression(&mut self, node: &'a ObjectExpression) -> Ty<'a> {
    let object = self.allocator.alloc(Record::default());

    for property in &node.properties {
      match property {
        ObjectPropertyKind::ObjectProperty(node) => {
          let key = self.exec_property_key(&node.key);
          let value = self.exec_expression(&node.value);
          let value = value;

          // tsc doesn't care. So we don't care either.
          // if matches!(&node.key, PropertyKey::StaticIdentifier(node) if node.name == "__proto__") {
          //   object.init_proto(value);
          // } else {

          let value = match node.kind {
            PropertyKind::Init => value,
            PropertyKind::Get | PropertyKind::Set => todo!(),
          };
          object.init_property(self, key, value, false, false);
        }
        ObjectPropertyKind::SpreadProperty(node) => {
          let argument = self.exec_expression(&node.argument);
          object.init_spread(self, argument);
        }
      }
    }

    Ty::Record(object)
  }
}
