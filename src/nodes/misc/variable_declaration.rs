use oxc::ast::ast::VariableDeclaration;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn declare_variable_declaration(&mut self, node: &'a VariableDeclaration<'a>) {
    for declarator in &node.declarations {
      self.declare_variable_declarator(declarator);
    }
  }

  pub fn init_variable_declaration(
    &mut self,
    node: &'a VariableDeclaration<'a>,
    loop_init: Option<Ty<'a>>,
  ) {
    if loop_init.is_some() {
      assert_eq!(node.declarations.len(), 1);
    }

    for declarator in &node.declarations {
      self.init_variable_declarator(declarator, loop_init);
    }
  }
}
