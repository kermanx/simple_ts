use oxc::semantic::SymbolId;
use oxc_index::{IndexVec, define_index_type};
use rustc_hash::FxHashMap;

use crate::Analyzer;

use super::{control::CfScopeKind, variable::Variable};

define_index_type! {
  pub struct RuntimeScopeId = u32;
}

#[derive(Debug)]
pub struct RuntimeScope<'a> {
  pub kind: CfScopeKind<'a>,
  pub exited: Option<bool>,
  pub variables: FxHashMap<SymbolId, Variable<'a>>,
}

#[derive(Debug, Default)]
pub struct RuntimeScopeTree<'a> {
  pub nodes: IndexVec<RuntimeScopeId, RuntimeScope<'a>>,
  pub stack: Vec<RuntimeScopeId>,
}

impl<'a> RuntimeScopeTree<'a> {
  pub fn push(&mut self, scope: RuntimeScope<'a>) -> RuntimeScopeId {
    let id = self.nodes.push(scope);
    self.stack.push(id);
    id
  }

  pub fn pop(&mut self) -> RuntimeScopeId {
    self.stack.pop().unwrap()
  }

  pub fn get(&self, id: RuntimeScopeId) -> &RuntimeScope<'a> {
    self.nodes.get(id).unwrap()
  }

  pub fn get_mut(&mut self, id: RuntimeScopeId) -> &mut RuntimeScope<'a> {
    self.nodes.get_mut(id).unwrap()
  }

  pub fn get_current_mut(&mut self) -> &mut RuntimeScope<'a> {
    self.get_mut(*self.stack.last().unwrap())
  }

  pub fn iter_stack(
    &self,
  ) -> impl DoubleEndedIterator<Item = &RuntimeScope<'a>> + ExactSizeIterator<Item = &RuntimeScope<'a>>
  {
    self.stack.iter().map(move |id| self.get(*id))
  }
}

impl<'a> Analyzer<'a> {
  pub fn push_scope(&mut self, kind: CfScopeKind<'a>) -> RuntimeScopeId {
    self.runtime_scopes.push(RuntimeScope {
      kind,
      exited: Some(false),
      variables: Default::default(),
    })
  }

  pub fn push_indeterminate_scope(&mut self) -> RuntimeScopeId {
    self.runtime_scopes.push(RuntimeScope {
      kind: CfScopeKind::Indeterminate,
      exited: None,
      variables: Default::default(),
    })
  }

  pub fn push_exit_blocker_scope(&mut self) -> RuntimeScopeId {
    self.runtime_scopes.push(RuntimeScope {
      kind: CfScopeKind::ExitBlocker(None),
      exited: None,
      variables: Default::default(),
    })
  }

  pub fn push_loop_scope(&mut self) -> RuntimeScopeId {
    self.runtime_scopes.push(RuntimeScope {
      kind: CfScopeKind::Loop,
      exited: None,
      variables: Default::default(),
    })
  }

  pub fn pop_scope(&mut self) {
    let id = self.runtime_scopes.pop();
    let scope = self.runtime_scopes.get(id);
    debug_assert!(scope.kind.get_blocked_exit().is_none());
    if !scope.kind.is_function() {
      self.apply_shadows([id], false);
    }
  }

  pub fn finalize_complementary_scopes(
    &mut self,
    scope_1: RuntimeScopeId,
    scope_2: RuntimeScopeId,
  ) {
    self.apply_shadows([scope_1, scope_2], true);
    self.apply_complementary_blocked_exits(scope_1, scope_2);
  }
}
