use super::cf::CfScopeKind;
use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::semantic::ScopeId;

pub struct CallScope<'a> {
  pub old_variable_scope_stack: Vec<ScopeId>,
  pub body_scope: (ScopeId, usize),

  pub is_async: bool,
  pub is_generator: bool,

  pub this: Ty<'a>,
  pub returned_values: Vec<Ty<'a>>,

  #[cfg(feature = "flame")]
  pub scope_guard: flame::SpanGuard,
}

impl<'a> CallScope<'a> {
  pub fn new(
    old_variable_scope_stack: Vec<ScopeId>,
    body_scope: (ScopeId, usize),
    is_async: bool,
    is_generator: bool,
    this: Ty<'a>,
  ) -> Self {
    CallScope {
      old_variable_scope_stack,
      body_scope,

      is_async,
      is_generator,

      this,
      returned_values: Vec::new(),

      #[cfg(feature = "flame")]
      scope_guard: flame::start_guard(callee.debug_name.to_string()),
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn push_call_scope(
    &mut self,
    variable_scope_stack: Vec<ScopeId>,
    is_async: bool,
    is_generator: bool,
    this: Ty<'a>,
  ) {
    let old_variable_scope_stack = self.scopes.replace_stack(variable_scope_stack);
    self.push_scope(CfScopeKind::Function);
    self.call_scopes.push(CallScope::new(
      old_variable_scope_stack,
      (self.scopes.current_id(), self.scopes.current_depth()),
      is_async,
      is_generator,
      this,
    ));
  }

  pub fn pop_call_scope(&mut self) -> Ty<'a> {
    let call_scope = self.call_scopes.pop().unwrap();
    self.scopes.replace_stack(call_scope.old_variable_scope_stack);

    if call_scope.is_async || call_scope.is_generator {
      todo!()
    } else {
      into_union(self.allocator, call_scope.returned_values)
    }
  }

  pub fn add_returned_value(&mut self, value: Ty<'a>) {
    self.call_scopes.last_mut().unwrap().returned_values.push(value);
  }
}
