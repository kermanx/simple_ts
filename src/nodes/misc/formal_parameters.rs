use crate::{analyzer::Analyzer, ast::DeclarationKind, r#type::Type};
use oxc::ast::ast::FormalParameters;

impl<'a> Analyzer<'a> {
  pub fn exec_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
    args: Type<'a>,
    kind: DeclarationKind,
  ) {
    let (elements_init, rest_init) =
      args.destruct_as_array(self, node.items.len(), node.rest.is_some());

    for param in &node.items {
      self.declare_binding_pattern(&param.pattern, false, kind);
    }

    for (param, init) in node.items.iter().zip(elements_init) {
      self.init_binding_pattern(&param.pattern, Some(init));
    }

    // In case of `function(x=arguments, y)`, `y` should also be consumed
    if self.call_scope_mut().need_consume_arguments {
      let arguments_consumed = self.consume_arguments();
      assert!(arguments_consumed);
    }

    if let Some(rest) = &node.rest {
      self.declare_binding_rest_element(rest, false, kind);
      self.init_binding_rest_element(rest, rest_init.unwrap());
    }
  }
}
