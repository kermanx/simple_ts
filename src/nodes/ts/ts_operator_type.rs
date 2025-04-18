use oxc::ast::ast::{TSType, TSTypeOperator, TSTypeOperatorOperator};

use crate::{
  Analyzer,
  ty::{Ty, unresolved::UnresolvedType},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_operator_type(&mut self, node: &'a TSTypeOperator<'a>) -> Ty<'a> {
    match node.operator {
      TSTypeOperatorOperator::Keyof => {
        let ty = self.resolve_type(&node.type_annotation);
        Ty::Unresolved(UnresolvedType::Keyof(self.allocator.alloc(ty)))
      }
      TSTypeOperatorOperator::Readonly => match &node.type_annotation {
        TSType::TSTupleType(node) => self.resolve_tuple_type(node, true),
        TSType::TSArrayType(node) => todo!(),
        _ => self.resolve_type(&node.type_annotation),
      },
      TSTypeOperatorOperator::Unique => match &node.type_annotation {
        TSType::TSSymbolKeyword(_) => todo!(),
        _ => self.resolve_type(&node.type_annotation),
      },
    }
  }
}
