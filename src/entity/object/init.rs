use super::{ObjectEntity, ObjectProperty, ObjectPropertyValue};
use crate::{
  analyzer::Analyzer,
  entity::{Entity, LiteralEntity},
};
use oxc::ast::ast::PropertyKind;

impl<'a> ObjectEntity<'a> {
  pub fn init_property(
    &self,
    analyzer: &mut Analyzer<'a>,
    kind: PropertyKind,
    key: Entity<'a>,
    value: Entity<'a>,
    definite: bool,
  ) {
    if let Some(key_literals) = key.get_to_literals(analyzer) {
      let definite = definite && key_literals.len() == 1;
      for key_literal in key_literals {
        match key_literal {
          LiteralEntity::String(key_str) => {
            let mut string_keyed = self.string_keyed.borrow_mut();
            let existing = string_keyed.get_mut(key_str);
            let reused_property = definite
              .then(|| {
                existing.as_ref().and_then(|existing| {
                  for property in existing.possible_values.iter() {
                    if let ObjectPropertyValue::Property(getter, setter) = property {
                      return Some((*getter, *setter));
                    }
                  }
                  None
                })
              })
              .flatten();

            let property_val = match kind {
              PropertyKind::Init => ObjectPropertyValue::Field(value, false),
              PropertyKind::Get => ObjectPropertyValue::Property(
                Some(value),
                reused_property.and_then(|(_, setter)| setter),
              ),
              PropertyKind::Set => ObjectPropertyValue::Property(
                reused_property.and_then(|(getter, _)| getter),
                Some(value),
              ),
            };
            let existing = string_keyed.get_mut(key_str);
            if definite || existing.is_none() {
              let property = ObjectProperty { definite, possible_values: vec![property_val] };
              string_keyed.insert(key_str, property);
            } else {
              existing.unwrap().possible_values.push(property_val);
            }
          }
          LiteralEntity::Symbol(key, _) => todo!(),
          _ => unreachable!("Invalid property key"),
        }
      }
    } else {
      let property_val = match kind {
        PropertyKind::Init => ObjectPropertyValue::Field(value, false),
        PropertyKind::Get => ObjectPropertyValue::Property(Some(value), None),
        PropertyKind::Set => ObjectPropertyValue::Property(None, Some(value)),
      };
      self.unknown_keyed.borrow_mut().possible_values.push(property_val);
    }
  }

  pub fn init_spread(&self, analyzer: &mut Analyzer<'a>, argument: Entity<'a>) {
    let properties = argument.enumerate_properties(analyzer);
    for (definite, key, value) in properties {
      self.init_property(analyzer, PropertyKind::Init, key, value, definite);
    }
  }

  pub fn init_rest(&self, property: ObjectPropertyValue<'a>) {
    let mut rest = self.rest.borrow_mut();
    if let Some(rest) = &mut *rest {
      rest.possible_values.push(property);
    } else {
      *rest = Some(ObjectProperty { definite: false, possible_values: vec![property] });
    }
  }
}
