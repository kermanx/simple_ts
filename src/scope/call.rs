use super::cf::CfScopeKind;
use crate::{
  analyzer::Analyzer,
  r#type::{union::into_union, Type},
};
use oxc::semantic::ScopeId;

pub struct CallScope<'a> {
  pub old_variable_scope_stack: Vec<ScopeId>,
  pub body_variable_scope: ScopeId,
  pub cf_scope_depth: usize,

  pub is_async: bool,
  pub is_generator: bool,

  pub returned_values: Vec<Type<'a>>,

  #[cfg(feature = "flame")]
  pub scope_guard: flame::SpanGuard,
}

impl<'a> CallScope<'a> {
  pub fn new(
    old_variable_scope_stack: Vec<ScopeId>,
    body_variable_scope: ScopeId,
    cf_scope_depth: usize,
    is_async: bool,
    is_generator: bool,
  ) -> Self {
    CallScope {
      old_variable_scope_stack,
      body_variable_scope,
      cf_scope_depth,

      is_async,
      is_generator,

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
  ) {
    let old_variable_scope_stack = self.variable_scopes.replace_stack(variable_scope_stack);
    let body_variable_scope = self.push_variable_scope();
    let body_cf_scope_depth = self.push_cf_scope(CfScopeKind::Function);
    self.call_scopes.push(CallScope::new(
      old_variable_scope_stack,
      body_variable_scope,
      body_cf_scope_depth,
      is_async,
      is_generator,
    ));
  }

  pub fn pop_call_scope(&mut self) -> Type<'a> {
    let call_scope = self.call_scopes.pop().unwrap();
    self.pop_variable_scope();
    self.variable_scopes.replace_stack(call_scope.old_variable_scope_stack);

    if call_scope.is_async || call_scope.is_generator {
      todo!()
    } else {
      into_union(self.allocator, call_scope.returned_values)
    }
  }

  pub fn add_returned_value(&mut self, value: Type<'a>) {
    self.call_scopes.last_mut().unwrap().returned_values.push(value);
  }
}
