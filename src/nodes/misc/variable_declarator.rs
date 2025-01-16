use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::VariableDeclarator;

impl<'a> Analyzer<'a> {
  pub fn declare_variable_declarator(&mut self, node: &'a VariableDeclarator) {
    self.declare_binding_pattern(&node.id, node.init.is_some());
  }

  pub fn init_variable_declarator(&mut self, node: &'a VariableDeclarator, init: Option<Ty<'a>>) {
    let mut init = init.or_else(|| node.init.as_ref().map(|init| self.exec_expression(init)));

    if !node.kind.is_const() {
      init = init.map(|init| self.get_widened_type(init));
    }

    self.init_binding_pattern(&node.id, init);
  }
}
