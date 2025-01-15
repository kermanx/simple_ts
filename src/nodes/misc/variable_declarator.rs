use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::VariableDeclarator;

impl<'a> Analyzer<'a> {
  pub fn declare_variable_declarator(&mut self, node: &'a VariableDeclarator) {
    self.declare_binding_pattern(&node.id, node.init.is_some());
  }

  pub fn init_variable_declarator(&mut self, node: &'a VariableDeclarator, init: Option<Ty<'a>>) {
    let init = match init {
      Some(init) => {
        // if node.init.is_some() {
        //   self.thrown_builtin_error(
        //     "for-in/for-of loop variable declaration may not have an initializer",
        //   );
        // }
        Some(init)
      }
      None => node.init.as_ref().map(|init| self.exec_expression(init)),
    };

    self.init_binding_pattern(&node.id, init);
  }
}
