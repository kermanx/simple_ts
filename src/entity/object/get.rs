use super::ObjectEntity;
use crate::{
  analyzer::Analyzer,
  entity::{consumed_object, Entity, LiteralEntity},
};

impl<'a> ObjectEntity<'a> {
  pub fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_property(self, analyzer, key);
    }

    analyzer.mark_object_property_exhaustive_read(self.cf_scope, self.object_id);

    let mut values = vec![];
    let mut getters = vec![];
    let mut definite = true;

    let mut check_rest = false;
    let mut may_add_undefined = false;
    let key = key.get_to_property_key(analyzer);
    if let Some(key_literals) = key.get_to_literals(analyzer) {
      let mut string_keyed = self.string_keyed.borrow_mut();
      for key_literal in key_literals {
        match key_literal {
          LiteralEntity::String(key_str) => {
            if let Some(property) = string_keyed.get_mut(key_str) {
              property.get(analyzer, &mut values, &mut getters, &mut definite);
            } else {
              check_rest = true;
              if let Some(val) = self.prototype.get_string_keyed(key_str) {
                values.push(val);
              } else {
                may_add_undefined = true;
              }
            }
          }
          LiteralEntity::Symbol(_, _) => todo!(),
          _ => unreachable!("Invalid property key"),
        }
      }

      check_rest |= !definite;
      may_add_undefined |= !definite;
    } else {
      for property in self.string_keyed.borrow_mut().values_mut() {
        property.get(analyzer, &mut values, &mut getters, &mut definite);
      }

      // TODO: prototype? Use a config IMO
      // Either:
      // - Skip prototype
      // - Return unknown and call all getters

      check_rest = true;
      may_add_undefined = true;
    }

    if check_rest {
      let mut rest = self.rest.borrow_mut();
      if let Some(rest) = &mut *rest {
        rest.get(analyzer, &mut values, &mut getters, &mut definite);
      } else if may_add_undefined {
        values.push(analyzer.factory.undefined);
      }
    }

    let indeterminate_getter = !values.is_empty() || getters.len() > 1 || !definite;

    {
      let mut unknown_keyed = self.unknown_keyed.borrow_mut();
      unknown_keyed.get(analyzer, &mut values, &mut getters, &mut definite);
    }

    if !getters.is_empty() {
      if indeterminate_getter {
        analyzer.push_indeterminate_cf_scope();
      }
      for getter in getters {
        values.push(getter.call_as_getter(analyzer, self));
      }
      if indeterminate_getter {
        analyzer.pop_cf_scope();
      }
    }

    analyzer.factory.try_union(values).unwrap_or(analyzer.factory.undefined)
  }
}
