use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::VariableDeclarator;

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
      let ty = self.resolve_type_annotation_or_defer(type_annotation);
      node.init.as_ref().map(|init| self.exec_expression(init, Some(ty)));
      Some(ty)
    } else if let Some(loop_init) = loop_init {
      Some(loop_init)
    } else {
      let mut ty = node.init.as_ref().map(|init| self.exec_expression(init, None));
      if !node.kind.is_const() {
        ty = ty.map(|ty| self.get_widened_type(ty));
      }
      ty
    };

    self.init_binding_pattern(&node.id, init);
  }
}
