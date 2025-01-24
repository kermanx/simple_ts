use oxc::ast::ast::Function;

use crate::{
  analyzer::Analyzer,
  scope::{call::CallScope, control::CfScopeKind},
  ty::{callable::CallableType, Ty},
};

impl<'a> Analyzer<'a> {
  pub fn exec_function(&mut self, node: &'a Function<'a>, _ty: Option<Ty<'a>>) -> Ty<'a> {
    let type_params = node
      .type_parameters
      .as_ref()
      .map(|type_parameters| self.resolve_type_parameter_declaration(&type_parameters))
      .unwrap_or_default();

    let body_scope = self.push_scope(CfScopeKind::Function);
    self.call_scopes.push(CallScope::new(body_scope, node.r#async, node.generator, annotated_ret));

    let (this_param, params, rest_param) = self.exec_formal_parameters(&node.params);

    let annotated_ret = node.return_type.as_ref().map(|n| &n.type_annotation);
    let inferred_ret = if let Some(body) = &node.body {
      let resolved_annotated = annotated_ret.map(|t| self.resolve_type(t));
      self.exec_function_body(body, node.r#async, node.generator, resolved_annotated)
    } else {
      Ty::Error
    };
    let return_type =
      annotated_ret.unwrap_or_else(|| self.allocator.alloc(self.serialize_type(inferred_ret)));

    Ty::Function(self.allocator.alloc(CallableType {
      bivariant: false,
      type_params,
      this_param,
      params,
      rest_param,
      return_type,
    }))
  }

  pub fn declare_function(&mut self, node: &'a Function<'a>) {
    let symbol = node.id.as_ref().unwrap().symbol_id.get().unwrap();
    let value = self.exec_function(node, None);

    self.declare_variable(symbol, true);
    self.init_variable(symbol, value);
  }
}
