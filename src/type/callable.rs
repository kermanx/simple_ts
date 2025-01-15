use super::{generic::GenericParam, Type};
use oxc::semantic::SymbolId;

#[derive(Debug, Clone)]
pub enum ReturnType<'a> {
  /// function f(): number;
  Type(Type<'a>),

  /// function f(a): asserts a is number;
  Assertion(SymbolId, Type<'a>),

  /// function f(a): a is number;
  Predicate(SymbolId, Type<'a>),
}

#[derive(Debug, Clone)]
pub struct Callable<'a, const CTOR: bool> {
  pub this_type: Option<Type<'a>>,
  pub type_params: Vec<GenericParam<'a>>,
  pub params: Vec<(bool, Type<'a>)>,
  pub return_type: Type<'a>,
}

pub type Function<'a> = Callable<'a, false>;
pub type Constructor<'a> = Callable<'a, true>;
