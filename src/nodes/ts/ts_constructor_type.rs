use oxc::ast::ast::TSConstructorType;

use crate::{
  Analyzer,
  ty::{Ty, callable::CallableType},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_constructor_type(&mut self, node: &'a TSConstructorType<'a>) -> Ty<'a> {
    let type_params = node
      .type_parameters
      .as_ref()
      .map(|type_params| self.resolve_type_parameter_declaration(type_params))
      .unwrap_or_default();
    let (_, params, rest_param) = self.resolve_formal_parameters(&node.params);
    let return_type = self.ctx_ty_from_ts_type(&node.return_type.type_annotation);

    Ty::Constructor(self.allocator.alloc(CallableType {
      is_method: false,
      scope: self.type_scopes.top(),
      type_params,
      this_param: None,
      params,
      rest_param,
      return_type,
    }))
  }
}
