use oxc::ast::ast::TSTypeName;

use crate::{Analyzer, ty::Ty};

use super::ts_qualified_name::NsOrTy;

impl<'a> Analyzer<'a> {
  pub fn resolve_type_name(&mut self, node: &'a TSTypeName<'a>) -> Ty<'a> {
    match node {
      TSTypeName::IdentifierReference(node) => self.resolve_type_identifier_reference(node),
      TSTypeName::QualifiedName(node) => match self.resolve_qualified_name(node) {
        NsOrTy::Ns(_) => Ty::Error,
        NsOrTy::Ty(ty) => ty,
      },
    }
  }
}
