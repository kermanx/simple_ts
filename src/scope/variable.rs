use oxc::semantic::{ScopeId, SymbolId};
use rustc_hash::FxHashMap;

use crate::{
  analyzer::Analyzer,
  ty::{unresolved::UnresolvedType, Ty},
};

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

impl<'a> Analyzer<'a> {
  pub fn declare_variable(&mut self, symbol: SymbolId, typed: bool) {
    if typed {
      self.variables.insert(symbol, Ty::Unresolved(UnresolvedType::UnInitVariable(symbol)));
    } else {
      self.scopes.get_current_mut().variables.insert(symbol, Variable::inferred(Ty::Undefined));
    }
  }

  pub fn init_variable(&mut self, symbol: SymbolId, value: Ty<'a>) {
    if let Some(resolved) = self.variables.get_mut(&symbol) {
      *resolved = value;
    } else {
      for depth in (0..self.scopes.stack.len()).rev() {
        let scope = self.scopes.get_mut_from_depth(depth);
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
      for scope in self.scopes.iter_stack().rev() {
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
      self
        .scopes
        .get_current_mut()
        .variables
        .entry(symbol)
        .and_modify(|variable| variable.value = value)
        .or_insert(Variable::shadow(value));
    }
  }

  pub fn apply_shadows<const N: usize>(&mut self, scopes: [ScopeId; N], complementary: bool) {
    let mut shadows: FxHashMap<SymbolId, Vec<Ty<'a>>> = FxHashMap::default();
    let mut len = 0;
    for scope in scopes {
      len += 1;
      let scope = self.scopes.get(scope);
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
      let value = self.into_union(values).unwrap();
      self.write_variable(symbol, value);
    }
  }

  fn is_symbol_var(&self, symbol: SymbolId) -> bool {
    self.semantic.symbols().get_flags(symbol).is_function_scoped_declaration()
  }
}
