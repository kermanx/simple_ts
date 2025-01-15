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
pub enum Ty<'a> {
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
  Union(&'a Vec<Ty<'a>>),
  Intersection(&'a Vec<Ty<'a>>),

  /* Generic */
  Generic(&'a Generic<'a>),
  Intrinsic(fn(Ty<'a>) -> Ty<'a>),
  /// Can only appear in inferred types
  UnresolvedType(&'a TSType<'a>),
  UnresolvedVariable(SymbolId),
}

impl<'a> Analyzer<'a> {
  pub fn get_facts(&mut self, ty: Ty<'a>) -> Facts {
    match ty {
      Ty::Error => Facts::NONE,

      Ty::Any => Facts::NONE,
      Ty::Unknown => Facts::NONE,
      Ty::Never => Facts::T_NE_ALL,
      Ty::Void => Facts::FALSY | Facts::T_NE_ALL,

      Ty::BigInt => Facts::T_EQ_BIGINT | Facts::T_NE_ALL & !Facts::T_EQ_BIGINT,
      Ty::Boolean => Facts::T_EQ_BOOLEAN | Facts::T_NE_ALL & !Facts::T_EQ_BOOLEAN,
      Ty::Null => {
        Facts::EQ_NULL
          | Facts::IS_NULLISH
          | Facts::FALSY
          | Facts::T_EQ_OBJECT
          | Facts::T_NE_ALL & !Facts::NE_NULL & !Facts::T_NE_OBJECT
      }
      Ty::Number => Facts::T_EQ_NUMBER | Facts::T_NE_ALL & !Facts::T_EQ_NUMBER,
      Ty::Object => Facts::T_EQ_OBJECT | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_OBJECT,
      Ty::String => Facts::T_EQ_STRING | Facts::T_NE_ALL & !Facts::T_EQ_STRING,
      Ty::Symbol => Facts::T_EQ_SYMBOL | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_SYMBOL,
      Ty::Undefined => {
        Facts::EQ_UNDEFINED
          | Facts::IS_NULLISH
          | Facts::FALSY
          | Facts::T_NE_ALL & !Facts::NE_UNDEFINED
      }

      Ty::StringLiteral(s) => self.get_facts(Ty::String) | Facts::truthy(s.len() > 0),
      Ty::NumericLiteral(n) => self.get_facts(Ty::Number) | Facts::truthy(n.0 != 0.0),
      Ty::BigIntLiteral(_) => self.get_facts(Ty::BigInt),
      Ty::BooleanLiteral(b) => self.get_facts(Ty::Boolean) | Facts::truthy(b),
      Ty::UniqueSymbol(_) => self.get_facts(Ty::Symbol),

      Ty::Record(_) => self.get_facts(Ty::Object),
      Ty::Function(_) | Ty::Constructor(_) => {
        Facts::T_EQ_FUNCTION | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_FUNCTION
      }
      Ty::Namespace(_) => self.get_facts(Ty::Object),

      Ty::Union(vals) => {
        let mut facts = Facts::all();
        for val in vals {
          facts &= self.get_facts(*val);
        }
        facts
      }
      Ty::Intersection(vals) => {
        let mut facts = Facts::empty();
        for val in vals {
          facts |= self.get_facts(*val);
        }
        facts
      }

      Ty::Generic(_) | Ty::Intrinsic(_) => {
        unreachable!("Cannot get facts of {ty:?}")
      }
      Ty::UnresolvedType(node) => {
        if let Some(resolved) = self.resolve_type(node) {
          self.get_facts(resolved)
        } else {
          Facts::NONE
        }
      }
      Ty::UnresolvedVariable(symbol) => match *self.variables.get(&symbol).unwrap() {
        Ty::UnresolvedVariable(s) if s == symbol => Facts::NONE,
        ty => self.get_facts(ty),
      },
    }
  }

  pub fn test_truthy(&mut self, ty: Ty<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::TRUTHY), facts.contains(Facts::FALSY)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("TRUTHY and FALSY are mutually exclusive"),
    }
  }

  pub fn test_nullish(&mut self, ty: Ty<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::IS_NULLISH), facts.contains(Facts::NOT_NULLISH)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("IS_NULLISH and NOT_NULLISH are mutually exclusive"),
    }
  }

  pub fn test_is_undefined(&mut self, ty: Ty<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::EQ_UNDEFINED), facts.contains(Facts::NE_UNDEFINED)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("EQ_UNDEFINED and NE_UNDEFINED are mutually exclusive"),
    }
  }

  pub fn get_property(&mut self, target: Ty<'a>, key: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn set_property(&mut self, target: Ty<'a>, key: Ty<'a>, value: Ty<'a>) {
    todo!()
  }

  pub fn delete_property(&mut self, target: Ty<'a>, key: Ty<'a>) {
    todo!()
  }

  pub fn enumerate_properties(self, analyzer: &mut Analyzer<'a>) -> Vec<(bool, Ty<'a>, Ty<'a>)> {
    todo!()
  }

  pub fn iterate_result_union(&mut self, target: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn destruct_as_array(
    &mut self,
    target: Ty<'a>,
    len: usize,
    need_rest: bool,
  ) -> (Vec<Ty<'a>>, Option<Ty<'a>>) {
    todo!()
  }

  pub fn get_to_numeric(&mut self, target: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn get_to_string(&mut self, target: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn get_to_boolean(&mut self, target: Ty<'a>) -> Ty<'a> {
    self.test_nullish(target).map_or(Ty::Boolean, Ty::BooleanLiteral)
  }

  pub fn get_to_awaited(&mut self, target: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn get_call_return(&mut self, target: Ty<'a>, this: Ty<'a>, arguments: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn get_instantiation_return(&mut self, target: Ty<'a>, arguments: Ty<'a>) -> Ty<'a> {
    todo!()
  }
}
