use oxc::ast::ast::TSTypeName;

use crate::{Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_name(&mut self, node: &'a TSTypeName<'a>) -> Ty<'a> {
    match node {
      TSTypeName::IdentifierReference(node) => self.resolve_type_identifier_reference(node),
      TSTypeName::QualifiedName(node) => self.resolve_qualified_name(node),
    }
  }
}
