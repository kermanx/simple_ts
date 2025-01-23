use oxc::{ast::ast::TSType, semantic::SymbolId};

use super::Ty;
use crate::Analyzer;

#[derive(Debug, Clone)]
pub struct UnresolvedConditionalType<'a> {
  pub check: Ty<'a>,
  pub extends: Ty<'a>,
  pub true_ty: Ty<'a>,
  pub false_ty: Ty<'a>,
}

#[derive(Debug, Clone)]
pub struct UnresolvedUnion<'a> {
  pub base: Ty<'a>,
  pub unresolved: Vec<UnresolvedType<'a>>,
}

#[derive(Debug, Clone)]
pub struct UnresolvedIntersection<'a> {
  pub base: Ty<'a>,
  pub unresolved: Vec<UnresolvedType<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub enum UnresolvedType<'a> {
  UnInitVariable(SymbolId),
  UnInitType(SymbolId),
  GenericParam(SymbolId),
  Conditional(&'a UnresolvedConditionalType<'a>),
  Keyof(&'a Ty<'a>),
  InferType(SymbolId),
}

impl<'a> Analyzer<'a> {
  pub fn resolve_unresolved(&mut self, ty: Ty<'a>) -> Ty<'a> {
    self.try_resolve_unresolved(ty).unwrap_or(ty)
  }

  /// Returns `None` if unchanged
  pub fn try_resolve_unresolved(&mut self, ty: Ty<'a>) -> Option<Ty<'a>> {
    todo!()
  }

  pub fn print_unresolved_type(&mut self, unresolved: UnresolvedType<'a>) -> TSType<'a> {
    todo!("{:#?}", unresolved)
  }
}
