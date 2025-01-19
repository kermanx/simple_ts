use crate::analyzer::Analyzer;
use oxc::ast::ast::IfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_if_statement(&mut self, node: &'a IfStatement) {
    self.exec_expression(&node.test, None);

    if node.alternate.is_some() {
      self.push_exit_blocker_scope();
    } else {
      self.push_indeterminate_scope();
    }

    self.exec_statement(&node.consequent);

    if let Some(alternate) = &node.alternate {
      let scope_1 = self.scopes.pop();

      self.push_exit_blocker_scope();
      self.exec_statement(alternate);
      let scope_2 = self.scopes.pop();

      self.finalize_complementary_scopes(scope_1, scope_2);
    } else {
      self.pop_scope();
    }
  }
}
