use super::{generic::GenericParam, Ty};
use crate::analyzer::Analyzer;
use oxc::{ast::ast::TSType, semantic::SymbolId};

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
pub struct CallableType<'a, const CTOR: bool> {
  pub type_params: Vec<GenericParam<'a>>,
  pub this_type: Ty<'a>,
  pub params: Vec<Ty<'a>>,
  pub rest_param: Option<Ty<'a>>,
  pub return_type: Ty<'a>,
}

pub type FunctionType<'a> = CallableType<'a, false>;
pub type ConstructorType<'a> = CallableType<'a, true>;

impl<'a> Analyzer<'a> {
  pub fn print_callable_type<const CTOR: bool>(
    &self,
    callable: &CallableType<'a, CTOR>,
  ) -> TSType<'a> {
    todo!()
  }
}
