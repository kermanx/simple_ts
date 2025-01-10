use crate::{analyzer::Analyzer, scope::CfScopeKind};
use oxc::ast::ast::IfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_if_statement(&mut self, node: &'a IfStatement) {
    let labels = self.take_labels();

    let test = self.exec_expression(&node.test).get_to_boolean(self);

    let (maybe_consequent, maybe_alternate) = match test.test_truthy() {
      Some(true) => (true, false),
      Some(false) => (false, true),
      None => (true, true),
    };

    let mut both_exit = true;
    let mut exit_target_inner = 0;
    let mut exit_target_outer = self.scope_context.cf.stack.len();

    if maybe_consequent {
      self.push_cf_scope(
        CfScopeKind::IfBranch,
        None,
        if maybe_alternate { None } else { Some(false) },
      );
      self.push_cf_scope(CfScopeKind::Labeled, labels.clone(), Some(false));
      self.exec_statement(&node.consequent);
      self.pop_cf_scope();
      let conditional_scope = self.pop_cf_scope_and_get_mut();
      if let Some(stopped_exit) = conditional_scope.blocked_exit {
        exit_target_inner = exit_target_inner.max(stopped_exit);
        exit_target_outer = exit_target_outer.min(stopped_exit);
      } else {
        both_exit = false;
      }
    }
    if maybe_alternate {
      self.push_cf_scope(
        CfScopeKind::IfBranch,
        None,
        if maybe_consequent { None } else { Some(false) },
      );
      if let Some(alternate) = &node.alternate {
        self.push_cf_scope(CfScopeKind::Labeled, labels.clone(), Some(false));
        self.exec_statement(alternate);
        self.pop_cf_scope();
        let conditional_scope = self.pop_cf_scope_and_get_mut();
        if let Some(stopped_exit) = conditional_scope.blocked_exit {
          exit_target_inner = exit_target_inner.max(stopped_exit);
          exit_target_outer = exit_target_outer.min(stopped_exit);
        } else {
          both_exit = false;
        }
      } else {
        self.pop_cf_scope();
        both_exit = false;
      }
    }

    if both_exit {
      if self.exit_to_impl(exit_target_inner, self.scope_context.cf.stack.len(), true) {
        self.exit_to_impl(exit_target_outer, exit_target_inner, false);
      }
    } else {
      self.exit_to_impl(exit_target_outer, self.scope_context.cf.stack.len(), false);
    }
  }
}
