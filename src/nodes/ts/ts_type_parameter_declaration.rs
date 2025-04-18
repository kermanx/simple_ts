use oxc::ast::ast::TSTypeParameterDeclaration;

use crate::{Analyzer, ty::generic::GenericParam};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_parameter_declaration(
    &mut self,
    node: &'a TSTypeParameterDeclaration<'a>,
  ) -> &'a [GenericParam<'a>] {
    self.allocator.alloc_slice(node.params.iter().map(|param| {
      let symbol_id = param.name.symbol_id();
      let constraint = param.constraint.as_ref().map(|c| self.ctx_ty_from_ts_type(c));
      if let Some(constraint) = constraint {
        self.generic_constraints.insert(symbol_id, constraint);
      }
      let default = param.default.as_ref().map(|d| self.ctx_ty_from_ts_type(d));
      GenericParam {
        symbol_id,
        constraint,
        default,
        r#in: param.r#in,
        out: param.out,
        r#const: param.r#const,
      }
    }))
  }
}
