use crate::{analyzer::Analyzer, ast::DeclarationKind, entity::Entity};
use oxc::ast::ast::BindingRestElement;

impl<'a> Analyzer<'a> {
  pub fn declare_binding_rest_element(
    &mut self,
    node: &'a BindingRestElement<'a>,
    exporting: bool,
    kind: DeclarationKind,
  ) {
    self.declare_binding_pattern(&node.argument, exporting, kind);
  }

  pub fn init_binding_rest_element(&mut self, node: &'a BindingRestElement<'a>, init: Entity<'a>) {
    self.init_binding_pattern(&node.argument, Some(init));
  }
}
