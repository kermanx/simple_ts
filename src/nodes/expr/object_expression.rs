use oxc::ast::ast::{ObjectExpression, ObjectPropertyKind, PropertyKind};

use crate::{
  analyzer::Analyzer,
  ty::{record::RecordTypeBuilder, Ty},
};

impl<'a> Analyzer<'a> {
  pub fn exec_object_expression(
    &mut self,
    node: &'a ObjectExpression,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let mut object = RecordTypeBuilder::default();

    for property in &node.properties {
      match property {
        ObjectPropertyKind::ObjectProperty(node) => {
          let key = self.exec_property_key(&node.key);
          let sat = sat.map(|sat| self.get_property(sat, key));
          let value = self.exec_expression(&node.value, sat);
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
          let argument = self.exec_expression(&node.argument, sat);
          object.init_spread(self, argument);
        }
      }
    }

    Ty::Record(self.allocator.alloc(object.build()))
  }
}
