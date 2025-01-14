use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::BindingRestElement;

impl<'a> Analyzer<'a> {
  pub fn declare_binding_rest_element(&mut self, node: &'a BindingRestElement<'a>, typed: bool) {
    self.declare_binding_pattern(&node.argument, typed);
  }

  pub fn init_binding_rest_element(&mut self, node: &'a BindingRestElement<'a>, init: Type<'a>) {
    self.init_binding_pattern(&node.argument, Some(init));
  }
}
