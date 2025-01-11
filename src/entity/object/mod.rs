mod delete;
mod enumerate;
mod get;
mod init;
mod property;
mod set;

use super::{
  consumed_object, Entity, EntityFactory, EntityTrait, EnumeratedProperties, IteratedElements,
  TypeofResult,
};
use crate::{analyzer::Analyzer, builtins::Prototype, use_consumed_flag};
use oxc::semantic::{ScopeId, SymbolId};
pub use property::{ObjectProperty, ObjectPropertyValue};
use rustc_hash::FxHashMap;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ObjectEntity<'a> {
  /// A built-in object is usually non-consumable
  pub consumable: bool,
  consumed: Cell<bool>,
  // deps: RefCell<ConsumableCollector<'a>>,
  /// Where the object is created
  cf_scope: ScopeId,
  pub object_id: SymbolId,
  pub prototype: &'a Prototype<'a>,

  /// Properties keyed by known string
  pub string_keyed: RefCell<FxHashMap<&'a str, ObjectProperty<'a>>>,
  /// Properties keyed by unknown value
  pub unknown_keyed: RefCell<ObjectProperty<'a>>,
  /// Properties keyed by unknown value, but not included in `string_keyed`
  pub rest: RefCell<Option<ObjectProperty<'a>>>,
  // TODO: symbol_keyed
}

impl<'a> EntityTrait<'a> for ObjectEntity<'a> {
  fn unknown_mutation(&'a self, analyzer: &mut Analyzer<'a>) {
    if !self.consumable {
      return;
    }

    use_consumed_flag!(self);

    // self.deps.take().consume_all(analyzer);

    for property in self.string_keyed.take().into_values() {
      property.consume(analyzer);
    }
    self.unknown_keyed.take().consume(analyzer);

    analyzer.mark_object_consumed(self.cf_scope, self.object_id);
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    self.get_property(analyzer, key)
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    self.set_property(analyzer, key, value);
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    self.enumerate_properties(analyzer)
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    self.delete_property(analyzer, key);
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::call(self, analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, props: Entity<'a>) -> Entity<'a> {
    consumed_object::jsx(self, analyzer, props)
  }

  fn r#await(&'a self, analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    self.unknown_mutation(analyzer);
    consumed_object::r#await(analyzer)
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    self.unknown_mutation(analyzer);
    consumed_object::iterate(analyzer)
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string_literal("object")
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    // FIXME: Special methods
    if self.consumed.get() {
      return consumed_object::get_to_string(analyzer);
    }
    analyzer.factory.string
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    // FIXME: Special methods
    if self.consumed.get() {
      return consumed_object::get_to_numeric(analyzer);
    }
    analyzer.factory.unknown
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.boolean_literal(true)
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.get_to_string(analyzer)
  }

  fn get_to_jsx_child(&'a self, _analyzer: &Analyzer<'a>) -> Entity<'a> {
    self
  }

  fn test_typeof(&self) -> TypeofResult {
    TypeofResult::Object
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(true)
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> Analyzer<'a> {
  pub fn new_empty_object(&mut self, prototype: &'a Prototype<'a>) -> &'a mut ObjectEntity<'a> {
    self.allocator.alloc(ObjectEntity {
      consumable: true,
      consumed: Cell::new(false),
      // deps: Default::default(),
      cf_scope: self.scope_context.cf.current_id(),
      object_id: self.scope_context.alloc_object_id(),
      string_keyed: RefCell::new(FxHashMap::default()),
      unknown_keyed: RefCell::new(ObjectProperty::default()),
      rest: RefCell::new(None),
      prototype,
    })
  }

  pub fn new_function_object(&mut self) -> &'a ObjectEntity<'a> {
    let object = self.new_empty_object(&self.builtins.prototypes.function);
    object.string_keyed.borrow_mut().insert(
      "prototype",
      ObjectProperty {
        definite: true,
        possible_values: vec![ObjectPropertyValue::Field(
          self.new_empty_object(&self.builtins.prototypes.object),
          false,
        )],
      },
    );
    self.allocator.alloc(object)
  }
}

impl<'a> EntityFactory<'a> {
  pub fn builtin_object(
    &self,
    object_id: SymbolId,
    prototype: &'a Prototype<'a>,
    consumable: bool,
  ) -> &'a mut ObjectEntity<'a> {
    self.alloc(ObjectEntity {
      consumable,
      consumed: Cell::new(false),
      // deps: Default::default(),
      cf_scope: ScopeId::new(0),
      object_id,
      string_keyed: Default::default(),
      unknown_keyed: Default::default(),
      rest: Default::default(),
      prototype,
    })
  }
}
