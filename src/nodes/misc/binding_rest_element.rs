use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::BindingRestElement;

impl<'a> Analyzer<'a> {
  pub fn declare_binding_rest_element(&mut self, node: &'a BindingRestElement<'a>, typed: bool) {
    self.declare_binding_pattern(&node.argument, typed);
  }

  pub fn init_binding_rest_element(
    &mut self,
    node: &'a BindingRestElement<'a>,
    init: Option<Ty<'a>>,
  ) -> Option<Ty<'a>> {
    self.init_binding_pattern(&node.argument, init)
  }
}
