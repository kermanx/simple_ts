use crate::{analyzer::Analyzer, ast::DeclarationKind, r#type::Type};
use oxc::ast::ast::CatchClause;

impl<'a> Analyzer<'a> {
  pub fn exec_catch_clause(&mut self, node: &'a CatchClause<'a>, value: Type<'a>) {
    self.push_indeterminate_cf_scope();

    if let Some(param) = &node.param {
      self.declare_binding_pattern(&param.pattern, false, DeclarationKind::Caught);
      self.init_binding_pattern(&param.pattern, Some(value));
    }

    self.exec_block_statement(&node.body);

    self.pop_cf_scope();
  }
}
