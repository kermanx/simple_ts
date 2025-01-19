use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TryStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_try_statement(&mut self, node: &'a TryStatement<'a>) {
    self.exec_block_statement(&node.block);

    if let Some(handler) = &node.handler {
      self.push_indeterminate_scope();

      if let Some(param) = &handler.param {
        self.declare_binding_pattern(&param.pattern, true);
        let init = if let Some(type_annotation) = &param.pattern.type_annotation {
          self.resolve_type_annotation_or_defer(type_annotation)
        } else {
          Ty::Unknown
        };
        self.init_binding_pattern(&param.pattern, Some(init));
      }

      self.exec_block_statement(&handler.body);

      self.pop_scope();
    };

    if let Some(finalizer) = &node.finalizer {
      self.exec_block_statement(finalizer);
    }
  }
}
