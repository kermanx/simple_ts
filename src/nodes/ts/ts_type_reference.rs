use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::{TSTypeName, TSTypeReference};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_reference(&mut self, node: &'a TSTypeReference<'a>) -> Option<Ty<'a>> {
    let base = match &node.type_name {
      TSTypeName::IdentifierReference(node) => {
        let reference = self.semantic.symbols().get_reference(node.reference_id());
        if let Some(symbol) = reference.symbol_id() {
          *self.types.get(&symbol)?
        } else {
          // Unresolved symbol
          Ty::Any
        }
      }
      TSTypeName::QualifiedName(_node) => todo!(),
    };

    if let Some(type_parameters) = &node.type_parameters {
      let type_parameters = self.resolve_type_parameter_instantiation(type_parameters)?;
      self.instantiate_generic(base, type_parameters)
    } else {
      Some(base)
    }
  }
}
