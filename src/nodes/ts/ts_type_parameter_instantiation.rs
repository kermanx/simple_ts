use oxc::ast::ast::TSTypeParameterInstantiation;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_parameter_instantiation(
    &mut self,
    node: &'a TSTypeParameterInstantiation<'a>,
  ) -> Vec<Ty<'a>> {
    let mut result = vec![];
    for arg in &node.params {
      result.push(self.resolve_type(&arg));
    }
    result
  }
}
