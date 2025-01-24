use oxc::{
  ast::{
    ast::{TSType, TSTypeName, TSTypeOperatorOperator},
    NONE,
  },
  semantic::SymbolId,
  span::SPAN,
};
use oxc_index::Idx;

use super::Ty;
use crate::Analyzer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnresolvedType<'a> {
  UnInitVariable(SymbolId),
  UnInitType(SymbolId),
  GenericParam(SymbolId),
  Keyof(&'a Ty<'a>),
  InferType(SymbolId),
}

impl<'a> Analyzer<'a> {
  pub fn serialize_unresolved_type(&mut self, unresolved: UnresolvedType<'a>) -> TSType<'a> {
    match unresolved {
      UnresolvedType::UnInitVariable(symbol) => todo!(),
      UnresolvedType::UnInitType(symbol) => todo!(),
      UnresolvedType::GenericParam(symbol) => self.ast_builder.ts_type_type_reference(
        SPAN,
        TSTypeName::IdentifierReference(
          self.ast_builder.alloc(self.serialize_identifier_reference(symbol)),
        ),
        NONE,
      ),
      UnresolvedType::Keyof(ty) => self.ast_builder.ts_type_type_operator(
        SPAN,
        TSTypeOperatorOperator::Keyof,
        self.serialize_type(*ty),
      ),
      UnresolvedType::InferType(symbol) => self.ast_builder.ts_type_infer_type(
        SPAN,
        self.ast_builder.ts_type_parameter(
          SPAN,
          self.serialize_binding_identifier(symbol),
          None,
          None,
          false,
          false,
          false,
        ),
      ),
    }
  }
}

pub fn get_placeholder_ty<'a>(index: usize) -> Ty<'a> {
  Ty::Unresolved(UnresolvedType::GenericParam(SymbolId::from_usize(usize::MAX - 1024 + index)))
}
