use super::{
  consumed_object, Entity, EntityFactory, EntityTrait, EnumeratedProperties, IteratedElements,
  TypeofResult,
};
use crate::{analyzer::Analyzer, builtins::Prototype, utils::F64WithEq};
use oxc::{allocator::Allocator, semantic::SymbolId};
use oxc_ecmascript::StringToNumber;
use oxc_syntax::number::ToJsString;
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LiteralEntity<'a> {
  String(&'a str),
  Number(F64WithEq, Option<&'a str>),
  BigInt(&'a str),
  Boolean(bool),
  Symbol(SymbolId, &'a str),
  Infinity(bool),
  NaN,
  Null,
  Undefined,
}

impl<'a> EntityTrait<'a> for LiteralEntity<'a> {
  fn unknown_mutation(&'a self, analyzer: &mut Analyzer<'a>) {
    // No effect
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    if matches!(self, LiteralEntity::Null | LiteralEntity::Undefined) {
      analyzer.thrown_builtin_error("Cannot get property of null or undefined");
      consumed_object::get_property(self, analyzer, key)
    } else {
      let prototype = self.get_prototype(analyzer);
      let key = key.get_to_property_key(analyzer);
      if let Some(key_literals) = key.get_to_literals(analyzer) {
        let mut values = vec![];
        for key_literal in key_literals {
          if let Some(property) = self.get_known_instance_property(analyzer, key_literal) {
            values.push(property);
          } else if let Some(property) = prototype.get_literal_keyed(key_literal) {
            values.push(property);
          } else {
            values.push(analyzer.factory.unmatched_prototype_property);
          }
        }
        analyzer.factory.union(values)
      } else {
        analyzer.factory.unknown
      }
    }
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    if matches!(self, LiteralEntity::Null | LiteralEntity::Undefined) {
      analyzer.thrown_builtin_error("Cannot set property of null or undefined");
      consumed_object::set_property(analyzer, key, value)
    } else {
      // No effect
    }
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    if let LiteralEntity::String(_) = self {
      analyzer.factory.unknown_string.enumerate_properties(analyzer)
    } else {
      // No effect
      vec![]
    }
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, _key: Entity<'a>) {
    if matches!(self, LiteralEntity::Null | LiteralEntity::Undefined) {
      analyzer.thrown_builtin_error("Cannot delete property of null or undefined");
    } else {
      // No effect
    }
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    analyzer.thrown_builtin_error(format!("Cannot call a non-function object {:?}", self));
    consumed_object::call(self, analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    analyzer.thrown_builtin_error(format!("Cannot construct a non-constructor object {:?}", self));
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, attributes: Entity<'a>) -> Entity<'a> {
    analyzer.factory.unknown
  }

  fn r#await(&'a self, analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    self
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    match self {
      LiteralEntity::String(value) => {
        (vec![], (!value.is_empty()).then_some(analyzer.factory.unknown_string))
      }
      _ => {
        self.unknown_mutation(analyzer);
        analyzer.thrown_builtin_error("Cannot iterate over a non-iterable object");
        consumed_object::iterate(analyzer)
      }
    }
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string(self.test_typeof().to_string().unwrap())
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.alloc(LiteralEntity::String(self.to_string(analyzer.allocator)))
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    match self {
      LiteralEntity::Number(_, _)
      | LiteralEntity::BigInt(_)
      | LiteralEntity::NaN
      | LiteralEntity::Infinity(_) => self,
      LiteralEntity::Boolean(value) => {
        if *value {
          analyzer.factory.number(1.0, Some("1"))
        } else {
          analyzer.factory.number(0.0, Some("0"))
        }
      }
      LiteralEntity::String(str) => {
        let val = str.string_to_number();
        if val.is_nan() {
          analyzer.factory.nan
        } else {
          analyzer.factory.number(val, None)
        }
      }
      LiteralEntity::Null => analyzer.factory.number(0.0, Some("0")),
      LiteralEntity::Symbol(_, _) => {
        // TODO: warn: TypeError: Cannot convert a Symbol value to a number
        analyzer.factory.unknown
      }
      LiteralEntity::Undefined => analyzer.factory.nan,
    }
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    match self.test_truthy() {
      Some(value) => analyzer.factory.boolean(value),
      None => analyzer.factory.unknown_boolean,
    }
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    match self {
      LiteralEntity::Symbol(_, _) => self,
      _ => self.get_to_string(analyzer),
    }
  }

  fn get_to_jsx_child(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if (TypeofResult::String | TypeofResult::Number).contains(self.test_typeof()) {
      self.get_to_string(analyzer)
    } else {
      analyzer.factory.string("")
    }
  }

  fn get_to_literals(&'a self, _analyzer: &Analyzer<'a>) -> Option<FxHashSet<LiteralEntity<'a>>> {
    let mut result = FxHashSet::default();
    result.insert(*self);
    Some(result)
  }

  fn get_literal(&'a self, _analyzer: &Analyzer<'a>) -> Option<LiteralEntity<'a>> {
    Some(*self)
  }

  fn test_typeof(&self) -> TypeofResult {
    match self {
      LiteralEntity::String(_) => TypeofResult::String,
      LiteralEntity::Number(_, _) => TypeofResult::Number,
      LiteralEntity::BigInt(_) => TypeofResult::BigInt,
      LiteralEntity::Boolean(_) => TypeofResult::Boolean,
      LiteralEntity::Symbol(_, _) => TypeofResult::Symbol,
      LiteralEntity::Infinity(_) => TypeofResult::Number,
      LiteralEntity::NaN => TypeofResult::Number,
      LiteralEntity::Null => TypeofResult::Object,
      LiteralEntity::Undefined => TypeofResult::Undefined,
    }
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(match self {
      LiteralEntity::String(value) => !value.is_empty(),
      LiteralEntity::Number(value, _) => *value != 0.0.into() && *value != (-0.0).into(),
      LiteralEntity::BigInt(value) => !value.chars().all(|c| c == '0'),
      LiteralEntity::Boolean(value) => *value,
      LiteralEntity::Symbol(_, _) => true,
      LiteralEntity::Infinity(_) => true,
      LiteralEntity::NaN | LiteralEntity::Null | LiteralEntity::Undefined => false,
    })
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(matches!(self, LiteralEntity::Null | LiteralEntity::Undefined))
  }
}

impl<'a> LiteralEntity<'a> {
  pub fn to_string(self, allocator: &'a Allocator) -> &'a str {
    match self {
      LiteralEntity::String(value) => value,
      LiteralEntity::Number(value, str_rep) => {
        str_rep.unwrap_or_else(|| allocator.alloc(value.0.to_js_string()))
      }
      LiteralEntity::BigInt(value) => value,
      LiteralEntity::Boolean(value) => {
        if value {
          "true"
        } else {
          "false"
        }
      }
      LiteralEntity::Symbol(_, str_rep) => str_rep,
      LiteralEntity::Infinity(positive) => {
        if positive {
          "Infinity"
        } else {
          "-Infinity"
        }
      }
      LiteralEntity::NaN => "NaN",
      LiteralEntity::Null => "null",
      LiteralEntity::Undefined => "undefined",
    }
  }

  // `None` for unresolvable, `Some(None)` for NaN, `Some(Some(value))` for number
  pub fn to_number(self) -> Option<Option<F64WithEq>> {
    match self {
      LiteralEntity::Number(value, _) => Some(Some(value)),
      LiteralEntity::BigInt(_value) => {
        // TODO: warn: TypeError: Cannot convert a BigInt value to a number
        None
      }
      LiteralEntity::Boolean(value) => Some(Some(if value { 1.0 } else { 0.0 }.into())),
      LiteralEntity::String(value) => {
        let value = value.trim();
        Some(if value.is_empty() {
          Some(0.0.into())
        } else if let Ok(value) = value.parse::<f64>() {
          Some(value.into())
        } else {
          None
        })
      }
      LiteralEntity::Null => Some(Some(0.0.into())),
      LiteralEntity::Symbol(_, _) => {
        // TODO: warn: TypeError: Cannot convert a Symbol value to a number
        None
      }
      LiteralEntity::NaN | LiteralEntity::Undefined => Some(None),
      LiteralEntity::Infinity(_) => None,
    }
  }

  fn get_prototype(&self, analyzer: &mut Analyzer<'a>) -> &'a Prototype<'a> {
    match self {
      LiteralEntity::String(_) => &analyzer.builtins.prototypes.string,
      LiteralEntity::Number(_, _) => &analyzer.builtins.prototypes.number,
      LiteralEntity::BigInt(_) => &analyzer.builtins.prototypes.bigint,
      LiteralEntity::Boolean(_) => &analyzer.builtins.prototypes.boolean,
      LiteralEntity::Symbol(_, _) => &analyzer.builtins.prototypes.symbol,
      LiteralEntity::Infinity(_) => &analyzer.builtins.prototypes.number,
      LiteralEntity::NaN => &analyzer.builtins.prototypes.number,
      LiteralEntity::Null | LiteralEntity::Undefined => {
        unreachable!("Cannot get prototype of null or undefined")
      }
    }
  }

  fn get_known_instance_property(
    &self,
    analyzer: &Analyzer<'a>,
    key: LiteralEntity<'a>,
  ) -> Option<Entity<'a>> {
    match self {
      LiteralEntity::String(value) => {
        let LiteralEntity::String(key) = key else { return None };
        if key == "length" {
          Some(analyzer.factory.number(value.len() as f64, None))
        } else if let Ok(index) = key.parse::<usize>() {
          Some(
            value
              .get(index..index + 1)
              .map_or(analyzer.factory.undefined, |v| analyzer.factory.string(v)),
          )
        } else {
          None
        }
      }
      _ => None,
    }
  }

  pub fn strict_eq(self, other: LiteralEntity) -> Option<bool> {
    // 0.0 === -0.0
    if let (LiteralEntity::Number(l, _), LiteralEntity::Number(r, _)) = (self, other) {
      let eq = if l == 0.0.into() || l == (-0.0).into() {
        r == 0.0.into() || r == (-0.0).into()
      } else {
        l == r
      };
      return Some(eq);
    }

    Some(self == other && self != LiteralEntity::NaN)
  }
}

impl<'a> EntityFactory<'a> {
  pub fn string(&self, value: &'a str) -> Entity<'a> {
    self.alloc(LiteralEntity::String(value))
  }

  pub fn number(&self, value: impl Into<F64WithEq>, str_rep: Option<&'a str>) -> Entity<'a> {
    self.alloc(LiteralEntity::Number(value.into(), str_rep))
  }

  pub fn big_int(&self, value: &'a str) -> Entity<'a> {
    self.alloc(LiteralEntity::BigInt(value))
  }

  pub fn boolean(&self, value: bool) -> Entity<'a> {
    self.alloc(LiteralEntity::Boolean(value))
  }

  pub fn boolean_maybe_unknown(&self, value: Option<bool>) -> Entity<'a> {
    if let Some(value) = value {
      self.boolean(value)
    } else {
      self.unknown_boolean
    }
  }

  pub fn infinity(&self, positive: bool) -> Entity<'a> {
    self.alloc(LiteralEntity::Infinity(positive))
  }

  pub fn symbol(&self, id: SymbolId, str_rep: &'a str) -> Entity<'a> {
    self.alloc(LiteralEntity::Symbol(id, str_rep))
  }
}
