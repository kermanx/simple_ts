use super::Type;
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
pub struct Callable<'a> {
  pub this: Type<'a>,
  pub parameters: Vec<(bool, Type<'a>)>,
  pub return_type: Type<'a>,
}
