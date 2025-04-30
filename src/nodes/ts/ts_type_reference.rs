use oxc::ast::ast::TSTypeReference;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_reference(&mut self, node: &'a TSTypeReference<'a>) -> Ty<'a> {
    let base = self.resolve_type_name_ty(&node.type_name);

    if let Some(type_arguments) = &node.type_arguments {
      let args = self.resolve_type_parameter_instantiation(type_arguments);
      self.create_generic_instance(base, args)
    } else {
      base
    }
  }
}
