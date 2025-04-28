use oxc::ast::ast::{TSQualifiedName, TSTypeName};

use crate::{
  Analyzer,
  ty::{Ty, property_key::PropertyKeyType, unresolved::UnresolvedType},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_qualified_name(&mut self, node: &'a TSQualifiedName<'a>) -> Ty<'a> {
    let left = match &node.left {
      TSTypeName::IdentifierReference(node) => {
        let reference = self.semantic.scoping().get_reference(node.reference_id());
        if let Some(symbol_id) = reference.symbol_id() {
          if let Some(namespace) = self.namespaces.get(&symbol_id) {
            *namespace
          } else {
            Ty::Unresolved(UnresolvedType::UnInitType(symbol_id))
          }
        } else {
          Ty::Any
        }
      }
      TSTypeName::QualifiedName(node) => self.resolve_qualified_name(node),
    };
    self.get_property(left, PropertyKeyType::StringLiteral(&node.right.name))
  }
}
