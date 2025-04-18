use oxc::ast::ast::TSInferType;

use crate::{
  Analyzer,
  ty::{Ty, unresolved::UnresolvedType},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_infer_type(&mut self, node: &'a TSInferType<'a>) -> Ty<'a> {
    Ty::Unresolved(UnresolvedType::InferType(node.type_parameter.name.symbol_id()))
  }
}
