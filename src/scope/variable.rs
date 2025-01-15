use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::semantic::{ScopeId, SymbolId};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy)]
pub struct Variable<'a> {
  pub is_shadow: bool,
  pub value: Ty<'a>,
}

impl<'a> Variable<'a> {
  pub fn inferred(value: Ty<'a>) -> Self {
    Self { is_shadow: false, value }
  }

  pub fn shadow(value: Ty<'a>) -> Self {
    Self { is_shadow: true, value }
  }
}

pub struct VariableScope<'a> {
  pub cf_scope: ScopeId,
  pub variables: FxHashMap<SymbolId, Variable<'a>>,
}

impl<'a> VariableScope<'a> {
  pub fn new(cf_scope: ScopeId) -> Self {
    Self { cf_scope, variables: FxHashMap::default() }
  }
}

impl<'a> Analyzer<'a> {
  pub fn push_variable_scope(&mut self) -> ScopeId {
    self.variable_scopes.push(VariableScope::new(self.cf_scopes.current_id()))
  }

  pub fn pop_variable_scope(&mut self) -> ScopeId {
    let id = self.variable_scopes.pop();
    self.apply_shadows([id], false);
    id
  }

  pub fn pop_variable_scope_no_apply_shadow(&mut self) -> ScopeId {
    self.variable_scopes.pop()
  }

  pub fn declare_variable(&mut self, symbol: SymbolId, typed: bool) {
    if typed {
      self.variables.insert(symbol, Ty::UnresolvedVariable(symbol));
    } else {
      self
        .variable_scopes
        .get_current_mut()
        .variables
        .insert(symbol, Variable::inferred(Ty::Undefined));
    }
  }

  pub fn init_variable(&mut self, symbol: SymbolId, value: Option<Ty<'a>>) {
    let value = if let Some(value) = value {
      value
    } else if self.is_symbol_var(symbol) {
      return;
    } else {
      Ty::Undefined
    };
    if let Some(resolved) = self.variables.get_mut(&symbol) {
      *resolved = value;
    } else {
      for depth in (0..self.variable_scopes.stack.len()).rev() {
        let scope = self.variable_scopes.get_mut_from_depth(depth);
        if let Some(variable) = scope.variables.get_mut(&symbol) {
          variable.value = value;
          return;
        }
      }
    }
  }

  pub fn read_variable(&self, symbol: SymbolId) -> Ty<'a> {
    if let Some(resolved) = self.variables.get(&symbol) {
      *resolved
    } else {
      for scope in self.variable_scopes.iter_stack().rev() {
        if let Some(variable) = scope.variables.get(&symbol) {
          return variable.value;
        }
      }
      if self.is_symbol_var(symbol) {
        // Var declaration like:
        // ```ts
        // read(a)
        // while (a) { var a; }
        // ```
        Ty::Any
      } else {
        unreachable!("Variable not found: {:?}", self.semantic.symbols().get_name(symbol));
      }
    }
  }

  pub fn write_variable(&mut self, symbol: SymbolId, value: Ty<'a>) {
    if let Some(_resolved) = self.variables.get_mut(&symbol) {
      // Do nothing
      // CHECKER: Should check type compatibility
    } else {
      let indeterminate = self.is_indeterminate_to(self.variable_scopes.get_current().cf_scope);

      if indeterminate {
        let allocator = self.allocator;
        if let Some(variable) = self.variable_scopes.get_current_mut().variables.get_mut(&symbol) {
          variable.value = into_union(allocator, [variable.value, value]);
        } else {
          let parent = self.read_variable(symbol);
          let value = into_union(allocator, [parent, value]);
          self.variable_scopes.get_current_mut().variables.insert(symbol, Variable::shadow(value));
        }
      } else {
        self
          .variable_scopes
          .get_current_mut()
          .variables
          .entry(symbol)
          .and_modify(|variable| variable.value = value)
          .or_insert(Variable::shadow(value));
      }
    }
  }

  pub fn apply_complementary_shadows(&mut self, scopes: impl IntoIterator<Item = ScopeId>) {
    self.apply_shadows(scopes, true);
  }

  fn apply_shadows(&mut self, scopes: impl IntoIterator<Item = ScopeId>, complementary: bool) {
    let mut shadows: FxHashMap<SymbolId, Vec<Ty<'a>>> = FxHashMap::default();
    let mut len = 0;
    for scope in scopes {
      len += 1;
      let scope = self.variable_scopes.get(scope);
      for (symbol, variable) in &scope.variables {
        if variable.is_shadow {
          shadows.entry(*symbol).or_default().push(variable.value);
        }
      }
    }
    for (symbol, mut values) in shadows {
      if !complementary || values.len() != len {
        values.push(self.read_variable(symbol));
      }
      let value = into_union(self.allocator, values);
      self.write_variable(symbol, value);
    }
  }

  fn is_symbol_var(&self, symbol: SymbolId) -> bool {
    self.semantic.symbols().get_flags(symbol).is_function_scoped_declaration()
  }
}
