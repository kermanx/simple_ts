use oxc::ast::ast::TSTypeName;

use crate::{
  Analyzer,
  ty::{Ty, namespace::Ns},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_name_ty(&mut self, node: &'a TSTypeName<'a>) -> Ty<'a> {
    match node {
      TSTypeName::IdentifierReference(node) => self.resolve_identifier_reference_ty(node),
      TSTypeName::QualifiedName(node) => self.resolve_qualified_name_ty(node),
    }
  }

  pub fn resolve_type_name_ns(&mut self, node: &'a TSTypeName<'a>) -> Option<&'a Ns<'a>> {
    match node {
      TSTypeName::IdentifierReference(node) => self.resolve_identifier_reference_ns(node),
      TSTypeName::QualifiedName(node) => self.resolve_qualified_name_ns(node),
    }
  }
}
