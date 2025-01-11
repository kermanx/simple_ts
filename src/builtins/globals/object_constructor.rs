use crate::{
  builtins::{constants::OBJECT_CONSTRUCTOR_OBJECT_ID, Builtins},
  init_namespace,
  r#type::{ObjectPropertyValue, Type, TypeofResult},
};
use std::borrow::BorrowMut;

impl<'a> Builtins<'a> {
  pub fn init_object_constructor(&mut self) {
    let factory = self.factory;

    let object =
      factory.builtin_object(OBJECT_CONSTRUCTOR_OBJECT_ID, &self.prototypes.function, false);
    object.init_rest(ObjectPropertyValue::Field(factory.unknown, true));

    init_namespace!(object, {
      "prototype" => factory.unknown,
      "assign" => self.create_object_assign_impl(),
      "keys" => self.create_object_keys_impl(),
      "values" => self.create_object_values_impl(),
      "entries" => self.create_object_entries_impl(),
    });

    self.globals.borrow_mut().insert("Object", object);
  }

  fn create_object_assign_impl(&self) -> Type<'a> {
    self.factory.implemented_builtin_fn("Object.assign", |analyzer, _, args| {
      let (known, rest) = args.iterate(analyzer);

      if known.len() < 2 {
        return analyzer.factory.unknown;
      }

      let target = known[0];

      let mut assign = |source: Type<'a>, indeterminate: bool| {
        let properties = source.enumerate_properties(analyzer);
        for (definite, key, value) in properties {
          if indeterminate || !definite {
            analyzer.push_indeterminate_cf_scope();
          }
          target.set_property(analyzer, key, value);
          if indeterminate || !definite {
            analyzer.pop_cf_scope();
          }
        }
      };

      for source in &known[1..] {
        assign(*source, false);
      }
      if let Some(rest) = rest {
        assign(rest, true);
      }

      target
    })
  }

  fn create_object_keys_impl(&self) -> Type<'a> {
    self.factory.implemented_builtin_fn("Object.keys", |analyzer, _, args| {
      let object = args.destruct_as_array(analyzer, 1, false).0[0];
      let properties = object.enumerate_properties(analyzer);

      let array = analyzer.new_empty_array();

      for (_, key, _) in properties {
        if key.test_typeof().contains(TypeofResult::String) {
          array.init_rest(key.get_to_string(analyzer));
        }
      }

      array
    })
  }

  fn create_object_values_impl(&self) -> Type<'a> {
    self.factory.implemented_builtin_fn("Object.values", |analyzer, _, args| {
      let object = args.destruct_as_array(analyzer, 1, false).0[0];
      let properties = object.enumerate_properties(analyzer);

      let array = analyzer.new_empty_array();

      for (_, _, value) in properties {
        array.init_rest(value);
      }

      array
    })
  }

  fn create_object_entries_impl(&self) -> Type<'a> {
    self.factory.implemented_builtin_fn("Object.entries", |analyzer, _, args| {
      let object = args.destruct_as_array(analyzer, 1, false).0[0];
      let properties = object.enumerate_properties(analyzer);

      let array = analyzer.new_empty_array();

      for (_, key, value) in properties {
        let entry = analyzer.new_empty_array();
        entry.push_element(key.get_to_string(analyzer));
        entry.push_element(value);
        array.init_rest(entry);
      }

      array
    })
  }
}
