use super::{
  consumed_object, Entity, EntityFactory, EntityTrait, EnumeratedProperties, IteratedElements,
  ObjectEntity, TypeofResult,
};
use crate::{analyzer::Analyzer, use_consumed_flag};
use oxc::{ast::ast::Class, semantic::ScopeId};
use std::{cell::Cell, rc::Rc};

#[derive(Debug)]
pub struct ClassEntity<'a> {
  consumed: Cell<bool>,
  pub node: &'a Class<'a>,
  pub keys: Vec<Option<Entity<'a>>>,
  statics: &'a ObjectEntity<'a>,
  pub super_class: Option<Entity<'a>>,
  pub variable_scope_stack: Rc<Vec<ScopeId>>,
}

impl<'a> EntityTrait<'a> for ClassEntity<'a> {
  fn unknown_mutation(&'a self, analyzer: &mut Analyzer<'a>) {
    use_consumed_flag!(self);

    self.statics.unknown_mutation(analyzer);
    analyzer.construct_class(self);
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_property(self, analyzer, key);
    }
    if analyzer.entity_op.strict_eq(
      analyzer,
      key.get_to_property_key(analyzer),
      analyzer.factory.string("prototype"),
    ) != Some(false)
    {
      self.unknown_mutation(analyzer);
      return consumed_object::get_property(self, analyzer, key);
    }
    self.statics.get_property(analyzer, key)
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    self.statics.set_property(analyzer, key, value)
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    self.statics.delete_property(analyzer, key)
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    self.statics.enumerate_properties(analyzer)
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    analyzer.thrown_builtin_error("Class constructor A cannot be invoked without 'new'");
    consumed_object::call(self, analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, props: Entity<'a>) -> Entity<'a> {
    consumed_object::jsx(self, analyzer, props)
  }

  fn r#await(&'a self, analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    // In case of `class A { static then() {} }`
    self.unknown_mutation(analyzer);
    consumed_object::r#await(analyzer)
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    self.unknown_mutation(analyzer);
    consumed_object::iterate(analyzer)
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string("function")
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_to_string(analyzer);
    }
    analyzer.factory.unknown_string
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_to_numeric(analyzer);
    }
    analyzer.factory.nan
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.boolean(true)
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.get_to_string(analyzer)
  }

  fn get_to_jsx_child(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      analyzer.factory.unknown
    } else {
      // TODO: analyzer.thrown_builtin_error("Functions are not valid JSX children");
      analyzer.factory.string("")
    }
  }

  fn test_typeof(&self) -> TypeofResult {
    TypeofResult::Function
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(true)
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> EntityFactory<'a> {
  pub fn class(
    &self,
    node: &'a Class<'a>,
    keys: Vec<Option<Entity<'a>>>,
    variable_scope_stack: Vec<ScopeId>,
    super_class: Option<Entity<'a>>,
    statics: &'a ObjectEntity<'a>,
  ) -> Entity<'a> {
    self.alloc(ClassEntity {
      consumed: Cell::new(false),
      node,
      keys,
      statics,
      variable_scope_stack: Rc::new(variable_scope_stack),
      super_class,
    })
  }
}
