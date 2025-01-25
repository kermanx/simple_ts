use oxc::ast::ast::VariableDeclarator;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn declare_variable_declarator(&mut self, node: &'a VariableDeclarator) {
    self
      .declare_binding_pattern(&node.id, node.init.is_some() || node.id.type_annotation.is_some());
  }

  pub fn init_variable_declarator(
    &mut self,
    node: &'a VariableDeclarator,
    loop_init: Option<Ty<'a>>,
  ) {
    let init = if let Some(type_annotation) = &node.id.type_annotation {
      let ty = self.resolve_type_annotation(type_annotation);
      node.init.as_ref().map(|init| self.exec_expression(init, Some(ty)));
      Some(ty)
    } else if let Some(loop_init) = loop_init {
      Some(loop_init)
    } else if let Some(init) = &node.init {
      if node.kind.is_const() && init.is_literal() {
        Some(self.exec_expression_with_as_const(init, None, true))
      } else {
        Some(self.exec_expression(init, None))
      }
    } else {
      None
    };

    self.init_binding_pattern(&node.id, init);
  }
}
