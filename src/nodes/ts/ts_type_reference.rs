use oxc::ast::ast::{IdentifierReference, TSTypeName, TSTypeReference};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_reference(&mut self, node: &'a TSTypeReference<'a>) -> Ty<'a> {
    let base = match &node.type_name {
      TSTypeName::IdentifierReference(node) => self.resolve_type_identifier_reference(node),
      TSTypeName::QualifiedName(_node) => todo!(),
    };

    if let Some(type_arguments) = &node.type_arguments {
      let args = self.resolve_type_parameter_instantiation(type_arguments);
      self.create_generic_instance(base, args)
    } else {
      base
    }
  }

  pub fn resolve_type_identifier_reference(&mut self, node: &'a IdentifierReference<'a>) -> Ty<'a> {
    let reference = self.semantic.scoping().get_reference(node.reference_id());
    if let Some(symbol_id) = reference.symbol_id() {
      self.type_scopes.search(symbol_id)
    } else {
      // TODO: Global type
      Ty::Unknown
    }
  }
}
