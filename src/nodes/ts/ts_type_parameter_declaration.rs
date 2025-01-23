use oxc::ast::ast::TSTypeParameterDeclaration;

use crate::{
  ty::{generic::GenericParam, unresolved::UnresolvedType, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_parameter_declaration(
    &mut self,
    node: &'a TSTypeParameterDeclaration<'a>,
  ) -> Vec<GenericParam<'a>> {
    for param in &node.params {
      let symbol_id = param.name.symbol_id();
      self.type_scopes.insert(symbol_id, Ty::Unresolved(UnresolvedType::GenericParam(symbol_id)));
    }
    node
      .params
      .iter()
      .map(|param| {
        let symbol_id = param.name.symbol_id();
        let constraint = param.constraint.as_ref().map(|c| self.resolve_type(c));
        if let Some(constraint) = constraint {
          self.generic_constraints.insert(symbol_id, constraint);
        }
        GenericParam {
          symbol_id,
          constraint,
          default: param.default.as_ref().map(|c| self.resolve_type(c)),
          r#in: param.r#in,
          out: param.out,
          r#const: param.r#const,
        }
      })
      .collect()
  }
}
