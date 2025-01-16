use crate::analyzer::Analyzer;
use oxc::ast::ast::IfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_if_statement(&mut self, node: &'a IfStatement) {
    self.exec_expression(&node.test);

    if node.alternate.is_some() {
      self.push_exit_blocker_scope();
    } else {
      self.push_indeterminate_scope();
    }

    self.exec_statement(&node.consequent);

    if let Some(alternate) = &node.alternate {
      let (blocked_1, shadow_1) = self.pop_scope_subtle();

      self.push_exit_blocker_scope();
      self.exec_statement(alternate);
      let (blocked_2, shadow_2) = self.pop_scope_subtle();

      self.apply_complementary_blocked_exits(blocked_1, blocked_2);
      self.apply_complementary_shadows([shadow_1, shadow_2]);
    } else {
      self.pop_scope();
    }
  }
}
