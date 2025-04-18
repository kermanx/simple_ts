use oxc::ast::ast::{ObjectExpression, ObjectPropertyKind, PropertyKind};

use crate::{
  analyzer::Analyzer,
  ty::{Ty, record::RecordTypeBuilder},
};

impl<'a> Analyzer<'a> {
  pub fn exec_object_expression(
    &mut self,
    node: &'a ObjectExpression,
    sat: Option<Ty<'a>>,
    as_const: bool,
  ) -> Ty<'a> {
    let mut object = RecordTypeBuilder::new_in(self.allocator);

    for property in &node.properties {
      match property {
        ObjectPropertyKind::ObjectProperty(node) => {
          let key = self.exec_property_key(&node.key);
          let sat = sat.map(|sat| self.get_property(sat, key));
          let value = self.exec_expression_with_as_const(&node.value, sat, as_const);

          // tsc doesn't care. So we don't care either.
          // if matches!(&node.key, PropertyKey::StaticIdentifier(node) if node.name == "__proto__") {
          //   object.init_proto(value);
          // } else {

          let value = match node.kind {
            PropertyKind::Init => value,
            PropertyKind::Get | PropertyKind::Set => todo!(),
          };
          object.init_property(self, key, value, false, as_const);
        }
        ObjectPropertyKind::SpreadProperty(node) => {
          let argument = self.exec_expression_with_as_const(&node.argument, sat, as_const);
          object.init_spread(self, argument);
        }
      }
    }

    Ty::Record(self.allocator.alloc(object.build()))
  }
}
