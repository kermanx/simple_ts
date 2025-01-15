pub mod callable;
pub mod facts;
pub mod generic;
pub mod namespace;
pub mod operations;
pub mod property_key;
pub mod record;
pub mod union;

use crate::{analyzer::Analyzer, utils::F64WithEq};
use callable::{Constructor, Function};
use facts::Facts;
use generic::Generic;
use namespace::Namespace;
use oxc::{ast::ast::TSType, semantic::SymbolId, span::Atom};
use record::Record;

#[derive(Debug, Clone, Copy)]
pub enum Type<'a> {
  Error,

  /* Intrinsics */
  Any,
  Unknown,
  Never,
  Void,

  /* Primitives */
  BigInt,
  Boolean,
  Null,
  Number,
  Object,
  String,
  Symbol,
  Undefined,

  /* Literals */
  StringLiteral(&'a Atom<'a>),
  NumericLiteral(F64WithEq),
  BigIntLiteral(&'a Atom<'a>),
  BooleanLiteral(bool),
  UniqueSymbol(SymbolId),

  /* Object like */
  Record(&'a Record<'a>),
  Function(&'a Function<'a>),
  Constructor(&'a Constructor<'a>),
  Namespace(&'a Namespace<'a>),

  /* Compound */
  Union(&'a Vec<Type<'a>>),
  Intersection(&'a Vec<Type<'a>>),

  /* Generic */
  Generic(&'a Generic<'a>),
  Intrinsic(fn(Type<'a>) -> Type<'a>),
  /// Can only appear in inferred types
  UnresolvedType(&'a TSType<'a>),
  UnresolvedVariable(SymbolId),
}

impl<'a> Analyzer<'a> {
  pub fn get_facts(&mut self, ty: Type<'a>) -> Facts {
    match ty {
      Type::Error => Facts::NONE,

      Type::Any => Facts::NONE,
      Type::Unknown => Facts::NONE,
      Type::Never => Facts::T_NE_ALL,
      Type::Void => Facts::FALSY | Facts::T_NE_ALL,

      Type::BigInt => Facts::T_EQ_BIGINT | Facts::T_NE_ALL & !Facts::T_EQ_BIGINT,
      Type::Boolean => Facts::T_EQ_BOOLEAN | Facts::T_NE_ALL & !Facts::T_EQ_BOOLEAN,
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

      Type::StringLiteral(s) => self.get_facts(Type::String) | Facts::truthy(s.len() > 0),
      Type::NumericLiteral(n) => self.get_facts(Type::Number) | Facts::truthy(n.0 != 0.0),
      Type::BigIntLiteral(_) => self.get_facts(Type::BigInt),
      Type::BooleanLiteral(b) => self.get_facts(Type::Boolean) | Facts::truthy(b),
      Type::UniqueSymbol(_) => self.get_facts(Type::Symbol),

      Type::Record(_) => self.get_facts(Type::Object),
      Type::Function(_) | Type::Constructor(_) => {
        Facts::T_EQ_FUNCTION | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_FUNCTION
      }
      Type::Namespace(_) => self.get_facts(Type::Object),

      Type::Union(vals) => {
        let mut facts = Facts::all();
        for val in vals {
          facts &= self.get_facts(*val);
        }
        facts
      }
      Type::Intersection(vals) => {
        let mut facts = Facts::empty();
        for val in vals {
          facts |= self.get_facts(*val);
        }
        facts
      }

      Type::Generic(_) | Type::Intrinsic(_) => {
        unreachable!("Cannot get facts of {ty:?}")
      }
      Type::UnresolvedType(node) => {
        if let Some(resolved) = self.resolve_type(node) {
          self.get_facts(resolved)
        } else {
          Facts::NONE
        }
      }
      Type::UnresolvedVariable(symbol) => match *self.variables.get(&symbol).unwrap() {
        Type::UnresolvedVariable(s) if s == symbol => Facts::NONE,
        ty => self.get_facts(ty),
      },
    }
  }

  pub fn test_truthy(&mut self, ty: Type<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::TRUTHY), facts.contains(Facts::FALSY)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("TRUTHY and FALSY are mutually exclusive"),
    }
  }

  pub fn test_nullish(&mut self, ty: Type<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::IS_NULLISH), facts.contains(Facts::NOT_NULLISH)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("IS_NULLISH and NOT_NULLISH are mutually exclusive"),
    }
  }

  pub fn test_is_undefined(&mut self, ty: Type<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::EQ_UNDEFINED), facts.contains(Facts::NE_UNDEFINED)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("EQ_UNDEFINED and NE_UNDEFINED are mutually exclusive"),
    }
  }

  pub fn get_property(&mut self, target: Type<'a>, key: Type<'a>) -> Type<'a> {
    todo!()
  }

  pub fn set_property(&mut self, target: Type<'a>, key: Type<'a>, value: Type<'a>) {
    todo!()
  }

  pub fn delete_property(&mut self, target: Type<'a>, key: Type<'a>) {
    todo!()
  }

  pub fn enumerate_properties(
    self,
    analyzer: &mut Analyzer<'a>,
  ) -> Vec<(bool, Type<'a>, Type<'a>)> {
    todo!()
  }

  pub fn iterate_result_union(&mut self, target: Type<'a>) -> Type<'a> {
    todo!()
  }

  pub fn destruct_as_array(
    &mut self,
    target: Type<'a>,
    len: usize,
    need_rest: bool,
  ) -> (Vec<Type<'a>>, Option<Type<'a>>) {
    todo!()
  }

  pub fn get_to_numeric(&mut self, target: Type<'a>) -> Type<'a> {
    todo!()
  }

  pub fn get_to_string(&mut self, target: Type<'a>) -> Type<'a> {
    todo!()
  }

  pub fn get_to_boolean(&mut self, target: Type<'a>) -> Type<'a> {
    self.test_nullish(target).map_or(Type::Boolean, Type::BooleanLiteral)
  }

  pub fn get_to_awaited(&mut self, target: Type<'a>) -> Type<'a> {
    todo!()
  }

  pub fn get_call_return(
    &mut self,
    target: Type<'a>,
    this: Type<'a>,
    arguments: Type<'a>,
  ) -> Type<'a> {
    todo!()
  }

  pub fn get_instantiation_return(&mut self, target: Type<'a>, arguments: Type<'a>) -> Type<'a> {
    todo!()
  }
}
