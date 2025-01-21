pub mod call;
pub mod cf;
pub mod tree;
pub mod variable;

use cf::CfScopeKind;
use oxc::semantic::{ScopeId, SymbolId};
use rustc_hash::FxHashMap;
use variable::Variable;

use crate::Analyzer;

#[derive(Debug)]
pub struct Scope<'a> {
  pub kind: CfScopeKind<'a>,
  pub exited: Option<bool>,
  pub variables: FxHashMap<SymbolId, Variable<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn push_scope(&mut self, kind: CfScopeKind<'a>) -> ScopeId {
    self.scopes.push(Scope { kind, exited: Some(false), variables: Default::default() })
  }

  pub fn push_indeterminate_scope(&mut self) -> ScopeId {
    self.scopes.push(Scope {
      kind: CfScopeKind::Indeterminate,
      exited: None,
      variables: Default::default(),
    })
  }

  pub fn push_exit_blocker_scope(&mut self) -> ScopeId {
    self.scopes.push(Scope {
      kind: CfScopeKind::ExitBlocker(None),
      exited: None,
      variables: Default::default(),
    })
  }

  pub fn push_loop_scope(&mut self) -> ScopeId {
    self.scopes.push(Scope { kind: CfScopeKind::Loop, exited: None, variables: Default::default() })
  }

  pub fn pop_scope(&mut self) {
    let id = self.scopes.pop();
    let scope = self.scopes.get(id);
    debug_assert!(scope.kind.get_blocked_exit().is_none());
    if !scope.kind.is_function() {
      self.apply_shadows([id], false);
    }
  }

  pub fn finalize_complementary_scopes(&mut self, scope_1: ScopeId, scope_2: ScopeId) {
    self.apply_shadows([scope_1, scope_2], true);
    self.apply_complementary_blocked_exits(scope_1, scope_2);
  }
}
