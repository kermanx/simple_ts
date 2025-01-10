use super::{ObjectEntity, ObjectProperty, ObjectPropertyValue};
use crate::{
  analyzer::Analyzer,
  entity::{consumed_object, Entity, EntityTrait, LiteralEntity},
};

impl<'a> ObjectEntity<'a> {
  pub fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    if self.consumed.get() {
      return consumed_object::set_property(analyzer, key, value);
    }

    let (has_exhaustive, mut indeterminate) =
      analyzer.pre_mutate_object(self.cf_scope, self.object_id);

    if has_exhaustive {
      self.consume(analyzer);
      return consumed_object::set_property(analyzer, key, value);
    }

    let mut setters = vec![];

    {
      let unknown_keyed = self.unknown_keyed.borrow();
      for possible_value in &unknown_keyed.possible_values {
        if let ObjectPropertyValue::Property(_, setter) = possible_value {
          if let Some(setter) = setter {
            setters.push((true, *setter));
          }
          indeterminate = true;
        }
      }
    }

    let key = key.get_to_property_key(analyzer);

    if let Some(key_literals) = key.get_to_literals(analyzer) {
      let mut string_keyed = self.string_keyed.borrow_mut();
      let mut rest = self.rest.borrow_mut();

      indeterminate |= key_literals.len() > 1;

      for key_literal in key_literals {
        match key_literal {
          LiteralEntity::String(key_str) => {
            if let Some(property) = string_keyed.get_mut(key_str) {
              property.set(analyzer, indeterminate, value, &mut setters);
            } else if let Some(rest) = &mut *rest {
              rest.set(analyzer, true, value, &mut setters);
            } else {
              string_keyed.insert(
                key_str,
                ObjectProperty {
                  definite: !indeterminate,
                  possible_values: vec![ObjectPropertyValue::Field(value, false)],
                },
              );
            }
          }
          LiteralEntity::Symbol(_, _) => todo!(),
          _ => unreachable!("Invalid property key"),
        }
      }
    } else {
      indeterminate = true;

      let mut unknown_keyed = self.unknown_keyed.borrow_mut();
      unknown_keyed.possible_values.push(ObjectPropertyValue::Field(value, false));

      let mut string_keyed = self.string_keyed.borrow_mut();
      for property in string_keyed.values_mut() {
        property.set(analyzer, true, value, &mut setters);
      }

      if let Some(rest) = &mut *self.rest.borrow_mut() {
        rest.set(analyzer, true, value, &mut setters);
      }
    }

    if !setters.is_empty() {
      let indeterminate = indeterminate || setters.len() > 1 || setters[0].0;
      if indeterminate {
        analyzer.push_indeterminate_cf_scope();
      }
      for (_, setter) in setters {
        setter.call_as_setter(analyzer, self, value);
      }
      if indeterminate {
        analyzer.pop_cf_scope();
      }
    }
  }
}
