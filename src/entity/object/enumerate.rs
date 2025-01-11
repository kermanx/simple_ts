use super::ObjectEntity;
use crate::{
  analyzer::Analyzer,
  entity::{consumed_object, EnumeratedProperties},
};
use std::mem;

impl<'a> ObjectEntity<'a> {
  pub fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    if self.consumed.get() {
      return consumed_object::enumerate_properties(self, analyzer);
    }

    analyzer.mark_object_property_exhaustive_read(self.cf_scope, self.object_id);
    analyzer.push_indeterminate_cf_scope();

    let mut result = vec![];
    let mut definite = true;

    {
      let mut values = vec![];
      let mut getters = vec![];

      {
        let mut unknown_keyed = self.unknown_keyed.borrow_mut();
        unknown_keyed.get(analyzer, &mut values, &mut getters, &mut definite);
        if let Some(rest) = &mut *self.rest.borrow_mut() {
          rest.get(analyzer, &mut values, &mut getters, &mut definite);
        }
      }

      for getter in getters {
        values.push(getter.call_as_getter(analyzer, self));
      }

      if let Some(value) = analyzer.factory.try_union(values) {
        result.push((false, analyzer.factory.unknown, value));
      }
    }

    {
      let string_keyed = self.string_keyed.borrow();
      let keys = string_keyed.keys().cloned().collect::<Vec<_>>();
      mem::drop(string_keyed);
      for key in keys {
        let mut string_keyed = self.string_keyed.borrow_mut();
        let properties = string_keyed.get_mut(&key).unwrap();

        let key_entity = analyzer.factory.string_literal(key);

        let mut definite = true;
        let mut values = vec![];
        let mut getters = vec![];
        properties.get(analyzer, &mut values, &mut getters, &mut definite);
        mem::drop(string_keyed);
        for getter in getters {
          values.push(getter.call_as_getter(analyzer, self));
        }

        if let Some(value) = analyzer.factory.try_union(values) {
          result.push((definite, key_entity, value));
        }
      }
    }

    analyzer.pop_cf_scope();

    result
  }
}
