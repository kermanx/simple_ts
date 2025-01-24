use oxc::{ast::ast::TSType, semantic::SymbolId};

use super::Ty;
use crate::Analyzer;

#[derive(Debug, Clone, Copy)]
pub enum UnresolvedType<'a> {
  UnInitVariable(SymbolId),
  UnInitType(SymbolId),
  GenericParam(SymbolId),
  Keyof(&'a Ty<'a>),
  InferType(SymbolId),
}

impl<'a> Analyzer<'a> {
  pub fn serialize_unresolved_type(&mut self, unresolved: UnresolvedType<'a>) -> TSType<'a> {
    todo!("{:#?}", unresolved)
  }
}
