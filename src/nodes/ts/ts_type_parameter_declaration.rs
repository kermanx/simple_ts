use oxc::ast::ast::TSTypeParameterDeclaration;

use crate::{ty::generic::GenericParam, Analyzer};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_parameter_declaration(
    &mut self,
    node: &'a TSTypeParameterDeclaration<'a>,
  ) -> Vec<GenericParam<'a>> {
    node
      .params
      .iter()
      .map(|param| GenericParam {
        symbol_id: param.name.symbol_id(),
        constraint: param.constraint.as_ref().map(|c| self.resolve_type_or_defer(c)),
        default: param.default.as_ref().map(|c| self.resolve_type_or_defer(c)),
        r#in: param.r#in,
        out: param.out,
        r#const: param.r#const,
      })
      .collect()
  }
}
