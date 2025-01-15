use super::{generic::GenericParam, Ty};
use oxc::semantic::SymbolId;

#[derive(Debug, Clone)]
pub enum ReturnType<'a> {
  /// function f(): number;
  Simple(Ty<'a>),

  /// function f(a): asserts a is number;
  Assertion(SymbolId, Ty<'a>),

  /// function f(a): a is number;
  Predicate(SymbolId, Ty<'a>),
}

#[derive(Debug, Clone)]
pub struct Callable<'a, const CTOR: bool> {
  pub this_type: Option<Ty<'a>>,
  pub type_params: Vec<GenericParam<'a>>,
  pub params: Vec<(bool, Ty<'a>)>,
  pub return_type: Ty<'a>,
}

pub type Function<'a> = Callable<'a, false>;
pub type Constructor<'a> = Callable<'a, true>;
