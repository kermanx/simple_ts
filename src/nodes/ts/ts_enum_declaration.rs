use oxc::ast::ast::TSEnumDeclaration;

use crate::{
  Analyzer,
  ty::{
    Ty,
    r#enum::{EnumClassId, EnumClassType, EnumMemberType},
    facts::Facts,
    namespace::Ns,
    record::RecordPropertyValue,
    union::UnionTypeBuilder,
  },
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_enum(&mut self, node: &'a TSEnumDeclaration<'a>) {
    let ns = self.allocator.alloc(Ns::new_in(self.allocator));
    let class = self.allocator.alloc(EnumClassType {
      id: EnumClassId::from_usize(self.enum_classes.len()),
      name: node.id.name,
      record: ns.record(),
      number_value: false,
      string_value: false,
    });
    let mut ns_ref = ns.borrow_mut();
    // FIXME: get rid of this
    self.enum_classes.push(unsafe { &*(class as *const _) });

    let mut counter = Some(0f64);
    let mut value_union = UnionTypeBuilder::default();
    for member in &node.body.members {
      let value = if let Some(initializer) = &member.initializer {
        self.exec_expression(initializer, None)
      } else if let Some(counter) = counter {
        Ty::NumericLiteral(counter.into())
      } else {
        Ty::Error
      };
      if let Ty::NumericLiteral(value) = value {
        counter = Some((value.0 + 1.0).into());
      } else {
        counter = None;
      }
      class.number_value |= self.get_facts(value).contains(Facts::T_EQ_NUMBER);
      class.string_value |= self.get_facts(value).contains(Facts::T_EQ_STRING);
      value_union.add(self, value);

      let id = member.id.static_name();
      let value = Ty::EnumMember(self.allocator.alloc(EnumMemberType {
        class: class.id,
        name: Some(id),
        value,
      }));

      ns_ref.record.string_keyed.init(
        self,
        id.as_str(),
        RecordPropertyValue { value, optional: false, readonly: false },
      );
    }
    self.accumulate_type(&node.id, Ty::EnumClass(class));

    self.declare_binding_identifier(&node.id, true);
    self.init_binding_identifier(&node.id, Some(Ty::EnumClass(class)));

    self.namespaces.insert(node.id.symbol_id(), ns);
    self.type_scopes.insert_on_top(
      node.id.symbol_id(),
      Ty::EnumMember(self.allocator.alloc(EnumMemberType {
        class: class.id,
        name: None,
        value: value_union.build(self),
      })),
    );
  }
}
