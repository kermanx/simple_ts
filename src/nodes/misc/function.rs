use oxc::ast::ast::Function;

use crate::{
  analyzer::Analyzer,
  ty::{Ty, callable::CallableType},
};

impl<'a> Analyzer<'a> {
  pub fn exec_function(&mut self, node: &'a Function<'a>, _sat: Option<Ty<'a>>) -> Ty<'a> {
    let type_params = if let Some(type_params) = &node.type_parameters {
      self.resolve_type_parameter_declaration(type_params)
    } else {
      self.allocator.alloc([])
    };

    let (this_param, params, rest_param) = self.exec_formal_parameters(&node.params);

    let annotated_ret = node.return_type.as_ref().map(|n| &n.type_annotation);
    let inferred_ret = if let Some(body) = &node.body {
      let resolved_annotated = annotated_ret.map(|t| self.resolve_type(t));
      self.exec_function_body(
        body,
        node.r#async,
        node.generator,
        /*TODO:*/ None,
        resolved_annotated,
      )
    } else {
      Ty::Error
    };
    let return_type = self.ctx_ty_from_annotation(&node.return_type, Some(inferred_ret));

    Ty::Function(self.allocator.alloc(CallableType {
      is_method: false,
      scope: self.type_scopes.top(),
      type_params,
      this_param,
      params,
      rest_param,
      return_type,
    }))
  }

  pub fn declare_function(&mut self, node: &'a Function<'a>) {
    let value = self.exec_function(node, None);
    let id = node.id.as_ref().unwrap();
    self.declare_binding_identifier(id, true);
    self.init_binding_identifier(id, Some(value));
  }
}
