use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TSTypeParameterInstantiation;

impl<'a> Analyzer<'a> {
  pub fn resolve_type_parameter_instantiation(
    &mut self,
    node: &'a TSTypeParameterInstantiation<'a>,
  ) -> Option<Vec<Ty<'a>>> {
    let mut result = vec![];
    for arg in &node.params {
      result.push(self.resolve_type(&arg)?);
    }
    Some(result)
  }
}
