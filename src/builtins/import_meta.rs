use super::{constants::IMPORT_META_OBJECT_ID, prototypes::BuiltinPrototypes, Builtins};
use crate::r#type::{EntityFactory, ObjectProperty, ObjectPropertyValue, Type};
use oxc::allocator::Allocator;

impl<'a> Builtins<'a> {
  pub fn create_import_meta(
    allocator: &'a Allocator,
    prototypes: &'a BuiltinPrototypes<'a>,
  ) -> Type<'a> {
    let object = factory.builtin_object(IMPORT_META_OBJECT_ID, &prototypes.null, true);
    object.init_rest(ObjectPropertyValue::Property(Some(factory.unknown), Some(factory.unknown)));

    // import.meta.url
    object.string_keyed.borrow_mut().insert(
      "url",
      ObjectProperty {
        definite: true,
        possible_values: vec![ObjectPropertyValue::Property(
          Some(
            factory
              .implemented_builtin_fn("import.meta.url", |analyzer, _, _| analyzer.factory.string),
          ),
          None,
        )],
      },
    );

    object
  }
}
