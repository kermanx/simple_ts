pub mod accumulator;
pub mod callable;
pub mod facts;
pub mod generic;
pub mod get_property;
pub mod intersection;
pub mod intrinsics;
pub mod namespace;
pub mod operations;
pub mod print;
pub mod property_key;
pub mod record;
pub mod union;
pub mod unresolved;
pub mod widen;

use crate::{analyzer::Analyzer, utils::F64WithEq};
use callable::{ConstructorType, FunctionType};
use generic::GenericType;
use intersection::IntersectionType;
use intrinsics::IntrinsicType;
use namespace::NamespaceType;
use oxc::{semantic::SymbolId, span::Atom};
use property_key::PropertyKeyType;
use record::RecordType;
use std::{hash, mem};
use union::UnionType;
use unresolved::UnresolvedType;

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
  Record(&'a RecordType<'a>),
  Function(&'a FunctionType<'a>),
  Constructor(&'a ConstructorType<'a>),
  Namespace(&'a NamespaceType<'a>),

  /* Compound */
  Union(&'a UnionType<'a>),
  Intersection(&'a IntersectionType<'a>),

  /* Generic */
  Generic(&'a GenericType<'a>),
  Intrinsic(&'a IntrinsicType),

  Unresolved(&'a UnresolvedType<'a>),
}

impl<'a> PartialEq for Ty<'a> {
  fn eq(&self, other: &Self) -> bool {
    // Compare as binary data
    unsafe { mem::transmute::<_, u128>(*self) == mem::transmute::<_, u128>(*other) }
  }
}

impl<'a> Eq for Ty<'a> {}

impl<'a> hash::Hash for Ty<'a> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    unsafe { mem::transmute::<_, u128>(*self).hash(state) }
  }
}

impl<'a> Analyzer<'a> {
  pub fn set_property(&mut self, _target: Ty<'a>, _key: PropertyKeyType<'a>, _value: Ty<'a>) {
    // Do nothing
  }

  pub fn delete_property(&mut self, _target: Ty<'a>, _key: PropertyKeyType<'a>) {
    // Do nothing
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
}
