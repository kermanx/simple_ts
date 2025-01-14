use crate::analyzer::Analyzer;
use oxc::{ast::ast::LabeledStatement, semantic::ScopeId, span::Atom};

#[derive(Debug, Clone, Copy)]
pub enum CfScopeKind<'a> {
  Module,
  Labeled(&'a LabeledStatement<'a>),
  Function,
  Loop,
  Switch,
  If,

  Indeterminate,
  ExitBlocker(Option<usize>),
}

impl<'a> CfScopeKind<'a> {
  pub fn is_function(self) -> bool {
    matches!(self, CfScopeKind::Function)
  }

  pub fn is_breakable_without_label(self) -> bool {
    match self {
      CfScopeKind::Loop | CfScopeKind::Switch => true,
      _ => false,
    }
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
}

#[derive(Debug, Clone, Copy)]
pub struct CfScope<'a> {
  pub kind: CfScopeKind<'a>,
  pub exited: Option<bool>,
}

impl<'a> Analyzer<'a> {
  pub fn push_cf_scope(&mut self, kind: CfScopeKind<'a>) -> usize {
    self.cf_scopes.push(CfScope { kind, exited: Some(false) });
    self.cf_scopes.current_depth()
  }

  pub fn push_indeterminate_cf_scope(&mut self) -> usize {
    self.cf_scopes.push(CfScope { kind: CfScopeKind::Indeterminate, exited: None });
    self.cf_scopes.current_depth()
  }

  pub fn push_exit_blocker_cf_scope(&mut self) -> usize {
    self.cf_scopes.push(CfScope { kind: CfScopeKind::ExitBlocker(None), exited: None });
    self.cf_scopes.current_depth()
  }

  pub fn push_loop_cf_scope(&mut self) -> usize {
    self.cf_scopes.push(CfScope { kind: CfScopeKind::Loop, exited: None });
    self.cf_scopes.current_depth()
  }

  pub fn pop_cf_scope(&mut self) -> CfScope<'a> {
    let id = self.cf_scopes.pop();
    *self.cf_scopes.get(id)
  }

  pub fn pop_cf_scope_and_get_blocked_exit(&mut self) -> Option<usize> {
    if let CfScopeKind::ExitBlocker(target) = self.pop_cf_scope().kind {
      target
    } else {
      unreachable!()
    }
  }

  pub fn exit_to_impl(&mut self, from_depth: usize, target_depth: usize, mut must_exit: bool) {
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
}
