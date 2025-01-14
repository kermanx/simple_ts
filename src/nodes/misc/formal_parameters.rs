use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::FormalParameters;

impl<'a> Analyzer<'a> {
  pub fn exec_formal_parameters(&mut self, node: &'a FormalParameters<'a>, args: Type<'a>) {
    let (elements_init, rest_init) =
      self.destruct_as_array(args, node.items.len(), node.rest.is_some());

    for param in &node.items {
      self.declare_binding_pattern(&param.pattern, false);
    }

    for (param, init) in node.items.iter().zip(elements_init) {
      self.init_binding_pattern(&param.pattern, Some(init));
    }

    if let Some(rest) = &node.rest {
      self.declare_binding_rest_element(rest, false);
      self.init_binding_rest_element(rest, rest_init.unwrap());
    }
  }
}
