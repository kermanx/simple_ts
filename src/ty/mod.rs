pub mod accumulator;
pub mod callable;
pub mod ctx;
pub mod r#enum;
pub mod facts;
pub mod generic;
pub mod get_property;
pub mod interface;
pub mod intersection;
pub mod intrinsics;
pub mod lowest;
pub mod r#match;
pub mod namespace;
pub mod operations;
pub mod print;
pub mod property_key;
pub mod record;
pub mod tuple;
pub mod union;
pub mod unresolved;
pub mod widen;

use std::{hash, mem};

use callable::{ConstructorType, FunctionType};
use r#enum::{EnumClassType, EnumMemberType};
use generic::{GenericInstanceType, GenericType};
use interface::InterfaceType;
use intersection::IntersectionType;
use intrinsics::IntrinsicType;
use namespace::Ns;
use oxc::{semantic::SymbolId, span::Atom};
use property_key::PropertyKeyType;
use record::RecordType;
use tuple::TupleType;
use union::UnionType;
use unresolved::UnresolvedType;

use crate::{analyzer::Analyzer, utils::F64WithEq};

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
  Interface(&'a InterfaceType<'a>),
  Tuple(&'a TupleType<'a>),
  Function(&'a FunctionType<'a>),
  Constructor(&'a ConstructorType<'a>),

  /* Compound */
  Union(&'a UnionType<'a>),
  Intersection(&'a IntersectionType<'a>),

  /* Generic */
  Instance(&'a GenericInstanceType<'a>),
  // -- HKT starts here
  Generic(&'a GenericType<'a>),
  Intrinsic(&'a IntrinsicType),
  // -- HKT ends here

  /* Enum */
  EnumClass(&'a EnumClassType<'a>),
  EnumMember(&'a EnumMemberType<'a>),

  Unresolved(UnresolvedType<'a>),
}

impl PartialEq for Ty<'_> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Ty::Error, Ty::Error) => true,
      (Ty::Any, Ty::Any) => true,
      (Ty::Unknown, Ty::Unknown) => true,
      (Ty::Never, Ty::Never) => true,
      (Ty::Void, Ty::Void) => true,
      (Ty::BigInt, Ty::BigInt) => true,
      (Ty::Boolean, Ty::Boolean) => true,
      (Ty::Null, Ty::Null) => true,
      (Ty::Number, Ty::Number) => true,
      (Ty::Object, Ty::Object) => true,
      (Ty::String, Ty::String) => true,
      (Ty::Symbol, Ty::Symbol) => true,
      (Ty::Undefined, Ty::Undefined) => true,
      (Ty::StringLiteral(a), Ty::StringLiteral(b)) => a == b,
      (Ty::NumericLiteral(a), Ty::NumericLiteral(b)) => a == b,
      (Ty::BigIntLiteral(a), Ty::BigIntLiteral(b)) => a == b,
      (Ty::BooleanLiteral(a), Ty::BooleanLiteral(b)) => a == b,
      (Ty::UniqueSymbol(a), Ty::UniqueSymbol(b)) => a == b,
      (Ty::Record(a), Ty::Record(b)) => a as *const _ == b,
      (Ty::Interface(a), Ty::Interface(b)) => a as *const _ == b,
      (Ty::Tuple(a), Ty::Tuple(b)) => a as *const _ == b,
      (Ty::Function(a), Ty::Function(b)) => a as *const _ == b,
      (Ty::Constructor(a), Ty::Constructor(b)) => a as *const _ == b,
      (Ty::Union(a), Ty::Union(b)) => a as *const _ == b,
      (Ty::Intersection(a), Ty::Intersection(b)) => a as *const _ == b,
      (Ty::Instance(a), Ty::Instance(b)) => a as *const _ == b,
      (Ty::Generic(a), Ty::Generic(b)) => a as *const _ == b,
      (Ty::Intrinsic(a), Ty::Intrinsic(b)) => a as *const _ == b,
      (Ty::EnumClass(a), Ty::EnumClass(b)) => a as *const _ == b,
      (Ty::EnumMember(a), Ty::EnumMember(b)) => a as *const _ == b,
      (Ty::Unresolved(a), Ty::Unresolved(b)) => a == b,
      _ => false,
    }
  }
}

impl Eq for Ty<'_> {}

impl hash::Hash for Ty<'_> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    mem::discriminant(self).hash(state);
    match self {
      Ty::Error => {}
      Ty::Any => {}
      Ty::Unknown => {}
      Ty::Never => {}
      Ty::Void => {}
      Ty::BigInt => {}
      Ty::Boolean => {}
      Ty::Null => {}
      Ty::Number => {}
      Ty::Object => {}
      Ty::String => {}
      Ty::Symbol => {}
      Ty::Undefined => {}
      Ty::StringLiteral(atom) => atom.hash(state),
      Ty::NumericLiteral(f) => f.hash(state),
      Ty::BigIntLiteral(atom) => atom.hash(state),
      Ty::BooleanLiteral(b) => b.hash(state),
      Ty::UniqueSymbol(id) => id.hash(state),
      Ty::Record(r) => (r as *const _ as usize).hash(state),
      Ty::Interface(i) => (i as *const _ as usize).hash(state),
      Ty::Tuple(t) => (t as *const _ as usize).hash(state),
      Ty::Function(f) => (f as *const _ as usize).hash(state),
      Ty::Constructor(c) => (c as *const _ as usize).hash(state),
      Ty::Union(u) => (u as *const _ as usize).hash(state),
      Ty::Intersection(i) => (i as *const _ as usize).hash(state),
      Ty::Instance(i) => (i as *const _ as usize).hash(state),
      Ty::Generic(g) => (g as *const _ as usize).hash(state),
      Ty::Intrinsic(i) => (i as *const _ as usize).hash(state),
      Ty::EnumClass(e) => (e as *const _ as usize).hash(state),
      Ty::EnumMember(e) => (e as *const _ as usize).hash(state),
      Ty::Unresolved(u) => u.hash(state),
    }
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
