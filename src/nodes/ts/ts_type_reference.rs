use oxc::ast::ast::{TSTypeName, TSTypeReference};

use crate::{
  analyzer::Analyzer,
  ty::{generic::GenericInstanceType, Ty},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_reference(&mut self, node: &'a TSTypeReference<'a>) -> Ty<'a> {
    let base = match &node.type_name {
      TSTypeName::IdentifierReference(node) => {
        let reference = self.semantic.symbols().get_reference(node.reference_id());
        self.read_type(reference.symbol_id())
      }
      TSTypeName::QualifiedName(_node) => todo!(),
    };

    if let Some(type_parameters) = &node.type_parameters {
      let type_parameters = self.resolve_type_parameter_instantiation(type_parameters);
      self.create_generic_instance(base, type_parameters)
    } else {
      base
    }
  }
}
