use oxc::{ast::ast::LabeledStatement, semantic::ScopeId, span::Atom};

use crate::analyzer::Analyzer;

#[derive(Debug, Clone, Copy)]
pub enum CfScopeKind<'a> {
  Module,
  Labeled(&'a LabeledStatement<'a>),
  Function,
  Loop,
  Switch,

  Indeterminate,
  ExitBlocker(Option<usize>),
}

impl<'a> CfScopeKind<'a> {
  pub fn is_function(self) -> bool {
    matches!(self, CfScopeKind::Function)
  }

  pub fn is_breakable_without_label(self) -> bool {
    matches!(self, CfScopeKind::Loop | CfScopeKind::Switch)
  }

  pub fn is_continuable(self) -> bool {
    matches!(self, CfScopeKind::Loop)
  }

  pub fn matches_label(self, label: &'a Atom<'a>) -> bool {
    match self {
      CfScopeKind::Labeled(labeled) => labeled.label.name == label,
      _ => false,
    }
  }

  pub fn get_blocked_exit(self) -> Option<usize> {
    if let CfScopeKind::ExitBlocker(target) = self {
      target
    } else {
      None
    }
  }
}

impl<'a> Analyzer<'a> {
  fn exit_to_impl(&mut self, from_depth: usize, target_depth: usize, mut must_exit: bool) {
    for depth in (target_depth..from_depth).rev() {
      let id = self.cf_scopes.stack[depth];
      let cf_scope = self.cf_scopes.get_mut(id);

      // Update exited state
      if must_exit {
        let is_indeterminate = cf_scope.exited.is_none();
        cf_scope.exited = Some(true);

        // Stop exiting outer scopes if one inner scope is indeterminate.
        if is_indeterminate {
          must_exit = false;
          if let CfScopeKind::ExitBlocker(target) = &mut cf_scope.kind {
            // For the `if` statement, do not mark the outer scopes as indeterminate here.
            // Instead, let the `if` statement handle it.
            assert!(target.is_none());
            *target = Some(target_depth);
            return;
          }
        }
      } else {
        cf_scope.exited = None;
      }
    }
  }

  /// If the label is used, `true` is returned.
  pub fn break_to_label(&mut self, label: Option<&'a Atom<'a>>) -> bool {
    let mut is_closest_breakable = true;
    let mut target_depth = None;
    let mut label_used = false;
    for (idx, cf_scope) in self.cf_scopes.iter_stack().enumerate().rev() {
      if cf_scope.kind.is_function() {
        break;
      }
      let breakable_without_label = cf_scope.kind.is_breakable_without_label();
      if let Some(label) = label {
        if cf_scope.kind.matches_label(label) {
          if !is_closest_breakable || !breakable_without_label {
            label_used = true;
          }
          target_depth = Some(idx);
          break;
        }
        if breakable_without_label {
          is_closest_breakable = false;
        }
      } else if breakable_without_label {
        target_depth = Some(idx);
        break;
      }
    }
    self.exit_to(target_depth.unwrap());
    label_used
  }

  pub fn exit_to(&mut self, target_depth: usize) {
    self.exit_to_impl(target_depth, self.cf_scopes.stack.len(), true);
  }

  pub fn exit_to_not_must(&mut self, target_depth: usize) {
    self.exit_to_impl(target_depth, self.cf_scopes.stack.len(), false);
  }

  /// If the label is used, `true` is returned.
  pub fn continue_to_label(&mut self, label: Option<&'a Atom<'a>>) -> bool {
    let mut is_closest_continuable = true;
    let mut target_depth = None;
    let mut label_used = false;
    for (idx, cf_scope) in self.cf_scopes.iter_stack().enumerate().rev() {
      if cf_scope.kind.is_function() {
        break;
      }
      let is_continuable = cf_scope.kind.is_continuable();
      if let Some(label) = label {
        if is_continuable {
          if cf_scope.kind.matches_label(label) {
            if !is_closest_continuable {
              label_used = true;
            }
            target_depth = Some(idx);
            break;
          }
          is_closest_continuable = false;
        }
      } else if is_continuable {
        target_depth = Some(idx);
        break;
      }
    }
    self.exit_to(target_depth.unwrap());
    label_used
  }

  pub fn is_indeterminate_to(&self, target: ScopeId) -> bool {
    let first_different = self.cf_scopes.find_lca(target).0 + 1;
    for depth in first_different..self.cf_scopes.stack.len() {
      if self.cf_scopes.get_from_depth(depth).exited.is_none() {
        return true;
      }
    }
    false
  }

  pub fn apply_complementary_blocked_exits(&mut self, scope_1: ScopeId, scope_2: ScopeId) {
    let blocked_1 = self.cf_scopes.get(scope_1).kind.get_blocked_exit();
    let blocked_2 = self.cf_scopes.get(scope_2).kind.get_blocked_exit();
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
  }
}
