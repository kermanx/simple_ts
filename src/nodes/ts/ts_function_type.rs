use oxc::ast::ast::TSFunctionType;

use crate::{
  ty::{callable::CallableType, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn resolve_function_type(&mut self, node: &'a TSFunctionType<'a>) -> Ty<'a> {
    let type_params = node
      .type_parameters
      .as_ref()
      .map(|type_params| self.resolve_type_parameter_declaration(type_params))
      .unwrap_or_default();
    let this_param =
      node.this_param.as_ref().map(|this_param| self.resovle_this_parameter(this_param));
    let (_, params, rest_param) = self.resolve_formal_parameters(&node.params);
    let return_type = &node.return_type.type_annotation;

    Ty::Function(self.allocator.alloc(CallableType {
      bivariant: false,
      type_params,
      this_param,
      params,
      rest_param,
      return_type,
    }))
  }
}
