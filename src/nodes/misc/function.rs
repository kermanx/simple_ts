use oxc::ast::ast::Function;

use crate::{
  analyzer::Analyzer,
  ty::{callable::CallableType, Ty},
};

impl<'a> Analyzer<'a> {
  pub fn exec_function(&mut self, node: &'a Function<'a>, _ty: Option<Ty<'a>>) -> Ty<'a> {
    let type_params = node
      .type_parameters
      .as_ref()
      .map(|type_parameters| self.resolve_type_parameter_declaration(&type_parameters))
      .unwrap_or_default();

    let (this_param, params, rest_param) = self.exec_formal_parameters(&node.params);

    let annotated_ret = if let Some(return_type) = &node.return_type {
      Some(self.resolve_type_annotation(return_type))
    } else {
      None
    };

    let return_type = if let Some(body) = &node.body {
      self.exec_function_body(body, node.r#async, node.generator, this_param, annotated_ret)
    } else {
      todo!()
    };

    Ty::Function(self.allocator.alloc(CallableType {
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
