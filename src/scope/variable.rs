use crate::{analyzer::Analyzer, r#type::Type};
use oxc::semantic::{ScopeId, SymbolId};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy)]
pub struct Variable<'a> {
  pub is_shadow: bool,
  pub value: Type<'a>,
}

impl<'a> Variable<'a> {
  pub fn inferred(value: Type<'a>) -> Self {
    Self { is_shadow: false, value }
  }

  pub fn shadow(value: Type<'a>) -> Self {
    Self { is_shadow: true, value }
  }
}

#[derive(Default)]
pub struct VariableScope<'a> {
  pub variables: FxHashMap<SymbolId, Variable<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn push_variable_scope(&mut self) {
    self.variable_scopes.push(VariableScope::default());
  }

  pub fn pop_variable_scope(&mut self) -> ScopeId {
    self.variable_scopes.pop()
  }

  pub fn declare_variable(&mut self, symbol: SymbolId, resolvable: bool) {
    if resolvable {
      self.variables.insert(symbol, Type::UnresolvedSymbol(symbol));
    } else {
      self
        .variable_scopes
        .get_current_mut()
        .variables
        .insert(symbol, Variable::inferred(Type::Undefined));
    }
  }

  pub fn init_variable(&mut self, symbol: SymbolId, value: Option<Type<'a>>) {
    let value = if let Some(value) = value {
      value
    } else if self.is_symbol_var(symbol) {
      return;
    } else {
      Type::Undefined
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

  pub fn read_variable(&self, symbol: SymbolId) -> Type<'a> {
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
        Type::Any
      } else {
        unreachable!("Variable not found: {:?}", self.semantic.symbols().get_name(symbol));
      }
    }
  }

  pub fn write_variable(&mut self, symbol: SymbolId, value: Type<'a>) {
    if let Some(_resolved) = self.variables.get_mut(&symbol) {
      // Do nothing
      // CHECKER: Should check type compatibility
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

  pub fn apply_shadows(&mut self, scopes: impl IntoIterator<Item = ScopeId>, overrides: bool) {
    let mut shadows: FxHashMap<SymbolId, Vec<Type<'a>>> = FxHashMap::default();
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
      if !overrides || values.len() != len {
        values.push(self.read_variable(symbol));
      }
      let value = Type::Union(self.allocator.alloc(values));
      self.write_variable(symbol, value);
    }
  }

  fn is_symbol_var(&self, symbol: SymbolId) -> bool {
    self.semantic.symbols().get_flags(symbol).is_function_scoped_declaration()
  }
}
