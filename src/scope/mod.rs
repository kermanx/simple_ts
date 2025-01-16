pub mod call;
pub mod cf;
pub mod tree;
pub mod variable;

use crate::Analyzer;
use cf::CfScopeKind;
use oxc::semantic::{ScopeId, SymbolId};
use rustc_hash::FxHashMap;
use variable::Variable;

#[derive(Debug)]
pub struct Scope<'a> {
  pub kind: CfScopeKind<'a>,
  pub exited: Option<bool>,
  pub variables: FxHashMap<SymbolId, Variable<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn push_scope(&mut self, kind: CfScopeKind<'a>) -> usize {
    self.scopes.push(Scope { kind, exited: Some(false), variables: Default::default() });
    self.scopes.current_depth()
  }

  pub fn push_indeterminate_scope(&mut self) -> usize {
    self.scopes.push(Scope {
      kind: CfScopeKind::Indeterminate,
      exited: None,
      variables: Default::default(),
    });
    self.scopes.current_depth()
  }

  pub fn push_exit_blocker_scope(&mut self) -> usize {
    self.scopes.push(Scope {
      kind: CfScopeKind::ExitBlocker(None),
      exited: None,
      variables: Default::default(),
    });
    self.scopes.current_depth()
  }

  pub fn push_loop_scope(&mut self) -> usize {
    self.scopes.push(Scope {
      kind: CfScopeKind::Loop,
      exited: None,
      variables: Default::default(),
    });
    self.scopes.current_depth()
  }

  pub fn pop_scope(&mut self) {
    let (blocked_exit, id) = self.pop_scope_subtle();
    debug_assert!(blocked_exit.is_none());
    self.apply_shadows([id], false);
  }

  pub fn pop_scope_subtle(&mut self) -> (Option<usize>, ScopeId) {
    let id = self.scopes.pop();
    let blocked_exit =
      if let CfScopeKind::ExitBlocker(target) = self.scopes.get(id).kind { target } else { None };
    (blocked_exit, id)
  }
}
