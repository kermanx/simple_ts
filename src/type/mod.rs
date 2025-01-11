mod callable;
mod facts;
mod generic;
mod namespace;
mod record;

use crate::{analyzer::Analyzer, utils::F64WithEq};
use callable::Callable;
use facts::Facts;
use generic::Generic;
use namespace::Namespace;
use oxc::span::Atom;
use record::Record;

#[derive(Debug, Clone, Copy)]
pub enum Type<'a> {
  /* Intrinsics */
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

  /* Literals */
  StringLiteral(&'a Atom<'a>),
  NumberLiteral(F64WithEq),
  BigIntLiteral(&'a Atom<'a>),
  BooleanLiteral(bool),

  // UniqueSymbol(SymbolId),

  /* Object like */
  Record(&'a Record<'a>),
  Callable(&'a Callable<'a>),
  Namespace(&'a Namespace<'a>),

  /* Compound */
  Union(&'a Vec<Type<'a>>),
  Intersection(&'a Vec<Type<'a>>),

  /* Generic */
  Generic(&'a Generic<'a>),
  Intrinsic(fn(Type<'a>) -> Type<'a>),
}

impl<'a> Type<'a> {
  pub fn get_facts(self) -> Facts {
    match self {
      Type::Any => Facts::NONE,
      Type::BigInt => Facts::T_EQ_BIGINT | Facts::T_NE_ALL & !Facts::T_EQ_BIGINT,
      Type::Boolean => Facts::T_EQ_BOOLEAN | Facts::T_NE_ALL & !Facts::T_EQ_BOOLEAN,
      Type::Never => Facts::T_NE_ALL,
      Type::Null => {
        Facts::EQ_NULL
          | Facts::IS_NULLISH
          | Facts::FALSY
          | Facts::T_EQ_OBJECT
          | Facts::T_NE_ALL & !Facts::NE_NULL & !Facts::T_NE_OBJECT
      }
      Type::Number => Facts::T_EQ_NUMBER | Facts::T_NE_ALL & !Facts::T_EQ_NUMBER,
      Type::Object => Facts::T_EQ_OBJECT | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_OBJECT,
      Type::String => Facts::T_EQ_STRING | Facts::T_NE_ALL & !Facts::T_EQ_STRING,
      Type::Symbol => Facts::T_EQ_SYMBOL | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_SYMBOL,
      Type::Undefined => {
        Facts::EQ_UNDEFINED
          | Facts::IS_NULLISH
          | Facts::FALSY
          | Facts::T_NE_ALL & !Facts::NE_UNDEFINED
      }
      Type::Unknown => Facts::NONE,
      Type::Void => Facts::FALSY | Facts::T_NE_ALL,

      Type::StringLiteral(s) => Type::String.get_facts() | Facts::truthy(s.len() > 0),
      Type::NumberLiteral(n) => Type::Number.get_facts() | Facts::truthy(n.0 != 0.0),
      Type::BigIntLiteral(_) => Type::BigInt.get_facts(),
      Type::BooleanLiteral(b) => Type::Boolean.get_facts() | Facts::truthy(b),

      Type::Record(r) => Type::Object.get_facts(),
      Type::Callable(c) => {
        Facts::T_EQ_FUNCTION | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_FUNCTION
      }
      Type::Namespace(n) => Type::Object.get_facts(),

      Type::Union(vals) => {
        let mut facts = Facts::all();
        for val in vals {
          facts &= val.get_facts();
        }
        facts
      }
      Type::Intersection(vals) => {
        let mut facts = Facts::empty();
        for val in vals {
          facts |= val.get_facts();
        }
        facts
      }

      Type::Generic(_) | Type::Intrinsic(_) => unreachable!(),
    }
  }

  pub fn test_truthy(self) -> Option<bool> {
    let facts = self.get_facts();
    match (facts.contains(Facts::TRUTHY), facts.contains(Facts::FALSY)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("TRUTHY and FALSY are mutually exclusive"),
    }
  }

  pub fn test_nullish(self) -> Option<bool> {
    let facts = self.get_facts();
    match (facts.contains(Facts::IS_NULLISH), facts.contains(Facts::NOT_NULLISH)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("IS_NULLISH and NOT_NULLISH are mutually exclusive"),
    }
  }

  pub fn test_is_undefined(self) -> Option<bool> {
    let facts = self.get_facts();
    match (facts.contains(Facts::EQ_UNDEFINED), facts.contains(Facts::NE_UNDEFINED)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("EQ_UNDEFINED and NE_UNDEFINED are mutually exclusive"),
    }
  }

  pub fn get_property(self, analyzer: &mut Analyzer<'a>, key: Type<'a>) -> Type<'a> {
    todo!()
  }

  pub fn set_property(self, analyzer: &mut Analyzer<'a>, key: Type<'a>, value: Type<'a>) {
    todo!()
  }

  pub fn delete_property(self, analyzer: &mut Analyzer<'a>, key: Type<'a>) {
    todo!()
  }

  pub fn enumerate_properties(
    self,
    analyzer: &mut Analyzer<'a>,
  ) -> Vec<(bool, Type<'a>, Type<'a>)> {
    todo!()
  }

  pub fn iterate_result_union(self, analyzer: &mut Analyzer<'a>) -> Option<Type<'a>> {
    todo!()
  }

  pub fn destruct_as_array(
    self,
    analyzer: &mut Analyzer<'a>,
    len: usize,
    need_rest: bool,
  ) -> (Vec<Type<'a>>, Option<Type<'a>>) {
    todo!()
  }

  pub fn get_to_numeric(self, analyzer: &mut Analyzer<'a>) -> Type<'a> {
    todo!()
  }

  pub fn get_to_string(self, analyzer: &mut Analyzer<'a>) -> Type<'a> {
    todo!()
  }

  pub fn get_to_boolean(self, analyzer: &mut Analyzer<'a>) -> Type<'a> {
    self.test_nullish().map_or(Type::Boolean, Type::BooleanLiteral)
  }

  pub fn get_to_property_key(self, analyzer: &mut Analyzer<'a>) -> Type<'a> {
    todo!()
  }
}
