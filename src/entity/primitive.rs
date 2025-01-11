use super::{
  consumed_object, Entity, EntityTrait, EnumeratedProperties, IteratedElements, TypeofResult,
};
use crate::{analyzer::Analyzer, builtins::Prototype};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveEntity {
  Any,
  BigInt,
  Boolean,
  Never,
  Null,
  Number,
  Object,
  String,
  Symbol,
  Undefined,
  Unknown,
  Void,
}

impl<'a> EntityTrait<'a> for PrimitiveEntity {
  fn unknown_mutation(&'a self, _analyzer: &mut Analyzer<'a>) ,

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    // TODO: PrimitiveEntity::String
    if *self == PrimitiveEntity::Mixed || *self == PrimitiveEntity::String {
      analyzer.factory.unknown
    } else {
      let prototype = self.get_prototype(analyzer);
      prototype.get_property(analyzer, key)
    }
  }

  fn set_property(&'a self, _analyzer: &mut Analyzer<'a>, _key: Entity<'a>, _value: Entity<'a>) {
    // No effect
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    if *self == PrimitiveEntity::String {
      vec![(false, analyzer.factory.string, analyzer.factory.string)]
    } else {
      vec![]
    }
  }

  fn delete_property(&'a self, _analyzer: &mut Analyzer<'a>, _key: Entity<'a>) {
    // No effect
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    analyzer.thrown_builtin_error("Cannot call non-object");
    consumed_object::call(self, analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    analyzer.thrown_builtin_error("Cannot construct non-object");
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, _props: Entity<'a>) -> Entity<'a> {
    analyzer.factory.unknown
  }

  fn r#await(&'a self, _analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    self
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    if *self == PrimitiveEntity::String {
      return (vec![], Some(analyzer.factory.unknown));
    }
    analyzer.thrown_builtin_error("Cannot iterate non-object");
    self.unknown_mutation(analyzer);
    consumed_object::iterate(analyzer)
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if let Some(str) = self.test_typeof().to_string() {
      analyzer.factory.string_literal(str)
    } else {
      analyzer.factory.string
    }
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.unknown
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    match self.test_truthy() {
      Some(val) => analyzer.factory.boolean_literal(val),
      None => analyzer.factory.boolean,
    }
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.unknown
  }

  fn get_to_jsx_child(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if matches!(self, PrimitiveEntity::Mixed | PrimitiveEntity::String | PrimitiveEntity::Number) {
      analyzer.factory.string
    } else {
      analyzer.factory.string_literal("")
    }
  }

  fn test_typeof(&self) -> TypeofResult {
    match self {
      PrimitiveEntity::String => TypeofResult::String,
      PrimitiveEntity::Number => TypeofResult::Number,
      PrimitiveEntity::BigInt => TypeofResult::BigInt,
      PrimitiveEntity::Boolean => TypeofResult::Boolean,
      PrimitiveEntity::Symbol => TypeofResult::Symbol,
      PrimitiveEntity::Mixed => TypeofResult::_Unknown,
    }
  }

  fn test_truthy(&self) -> Option<bool> {
    match self {
      PrimitiveEntity::Symbol => Some(true),
      _ => None,
    }
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> PrimitiveEntity {
  fn get_prototype(&self, analyzer: &mut Analyzer<'a>) -> &'a Prototype<'a> {
    match self {
      PrimitiveEntity::String => &analyzer.builtins.prototypes.string,
      PrimitiveEntity::Number => &analyzer.builtins.prototypes.number,
      PrimitiveEntity::BigInt => &analyzer.builtins.prototypes.bigint,
      PrimitiveEntity::Boolean => &analyzer.builtins.prototypes.boolean,
      PrimitiveEntity::Symbol => &analyzer.builtins.prototypes.symbol,
      PrimitiveEntity::Mixed => unreachable!("Cannot get prototype of mixed primitive"),
    }
  }
}
