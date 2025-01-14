use crate::analyzer::Analyzer;
use oxc::ast::ast::IfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_if_statement(&mut self, node: &'a IfStatement) {
    self.exec_expression(&node.test);

    if node.alternate.is_some() {
      self.push_exit_blocker_cf_scope();
    }

    self.push_variable_scope();
    self.exec_statement(&node.consequent);
    let shadow_1 = self.pop_variable_scope();

    if let Some(alternate) = &node.alternate {
      let blocked_1 = self.pop_cf_scope_and_get_blocked_exit();
      self.push_exit_blocker_cf_scope();

      self.push_variable_scope();
      self.exec_statement(alternate);
      let shadow_2 = self.pop_variable_scope();
      self.apply_shadows([shadow_1, shadow_2], true);

      let blocked_2 = self.pop_cf_scope_and_get_blocked_exit();
      match (blocked_1, blocked_2) {
        (Some(blocked_1), Some(blocked_2)) => {
          let inner = blocked_1.max(blocked_2);
          let outer = blocked_1.min(blocked_2);
          self.exit_to_impl(self.cf_scopes.stack.len(), inner, true);
          self.exit_to_impl(inner, outer, false);
        }
        (Some(blocked), None) | (None, Some(blocked)) => {
          self.exit_to_impl(self.cf_scopes.stack.len(), blocked, false);
        }
        (None, None) => {}
      }
    } else {
      self.apply_shadows([shadow_1], false);
    }
  }
}
