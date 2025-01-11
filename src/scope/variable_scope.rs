use super::exhaustive::ExhaustiveCallback;
use crate::{analyzer::Analyzer, ast::DeclarationKind, entity::Entity, utils::ast::AstKind2};
use oxc::semantic::{ScopeId, SymbolId};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct Variable<'a> {
  pub kind: DeclarationKind,
  pub cf_scope: ScopeId,
  pub exhausted: bool,
  pub value: Option<Entity<'a>>,
  pub decl_node: AstKind2<'a>,
}

#[derive(Default)]
pub struct VariableScope<'a> {
  pub variables: FxHashMap<SymbolId, &'a RefCell<Variable<'a>>>,
  pub this: Option<Entity<'a>>,
  pub arguments: Option<(Entity<'a>, Vec<SymbolId>)>,
  pub exhaustive_callbacks: FxHashMap<SymbolId, FxHashSet<ExhaustiveCallback<'a>>>,
}

impl fmt::Debug for VariableScope<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut map = f.debug_map();
    for (k, v) in self.variables.iter() {
      let v = v.borrow();
      map.entry(&k, &format!("{:?} {}", v.kind, v.value.is_some()));
    }
    map.finish()
  }
}

impl<'a> VariableScope<'a> {
  pub fn new() -> Self {
    Self::default()
  }
}

impl<'a> Analyzer<'a> {
  fn declare_on_scope(
    &mut self,
    id: ScopeId,
    kind: DeclarationKind,
    symbol: SymbolId,
    decl_node: AstKind2<'a>,
    fn_value: Option<Entity<'a>>,
  ) {
    if let Some(variable) = self.scope_context.variable.get(id).variables.get(&symbol) {
      // Here we can't use kind.is_untracked() because this time we are declaring a variable
      let old_kind = variable.borrow().kind;

      if old_kind.is_untracked() {
        if let Some(val) = fn_value {
          val.unknown_mutation(self)
        }
        return;
      }

      if old_kind.is_shadowable() && kind.is_redeclarable() {
        // Redeclaration is sometimes allowed
        // var x = 1; var x = 2;
        // function f(x) { var x }
        let mut variable = variable.borrow_mut();
        variable.kind = kind;
        // FIXME: Not sure if this is correct - how to handle the first declaration?
        variable.decl_node = decl_node;
        drop(variable);
        if let Some(new_val) = fn_value {
          self.write_on_scope(id, symbol, new_val);
        }
      } else {
        // Re-declaration
      }
    } else {
      let has_fn_value = fn_value.is_some();
      let variable = self.allocator.alloc(RefCell::new(Variable {
        kind,
        cf_scope: if kind.is_var() {
          self.cf_scope_id_of_call_scope()
        } else {
          self.scope_context.cf.current_id()
        },
        exhausted: false,
        value: fn_value,
        decl_node,
      }));
      self.scope_context.variable.get_mut(id).variables.insert(symbol, variable);
      if has_fn_value {
        self.add_exhaustive_callbacks(false, (id, symbol));
      }
    }
  }

  fn init_on_scope(&mut self, id: ScopeId, symbol: SymbolId, value: Option<Entity<'a>>) {
    let variable = self.scope_context.variable.get_mut(id).variables.get_mut(&symbol).unwrap();

    let variable_ref = variable.borrow();
    if variable_ref.kind.is_redeclarable() {
      if let Some(value) = value {
        drop(variable_ref);
        self.write_on_scope(id, symbol, value);
      } else {
        // Do nothing
      }
    } else if !variable_ref.exhausted {
      drop(variable_ref);
      variable.borrow_mut().value = Some(value.unwrap_or(self.factory.undefined));
      self.add_exhaustive_callbacks(false, (id, symbol));
    }
  }

  /// None: not in this scope
  /// Some(None): in this scope, but TDZ
  /// Some(Some(val)): in this scope, and val is the value
  fn read_on_scope(&mut self, id: ScopeId, symbol: SymbolId) -> Option<Option<Entity<'a>>> {
    self.scope_context.variable.get(id).variables.get(&symbol).copied().map(|variable| {
      let variable_ref = variable.borrow();
      let value =
        variable_ref.value.or_else(|| variable_ref.kind.is_var().then(|| self.factory.undefined));

      let value = if variable_ref.exhausted {
        value
      } else {
        let target_cf_scope = self.find_first_different_cf_scope(variable_ref.cf_scope);
        drop(variable_ref);
        self.mark_exhaustive_read((id, symbol), target_cf_scope);
        value
      };

      if value.is_none() {
        // TDZ
        let variable_ref = variable.borrow();
        let target_cf_scope = self.find_first_different_cf_scope(variable_ref.cf_scope);
        self.handle_tdz(target_cf_scope);
      }

      value
    })
  }

  fn write_on_scope(&mut self, id: ScopeId, symbol: SymbolId, new_val: Entity<'a>) -> bool {
    if let Some(variable) = self.scope_context.variable.get(id).variables.get(&symbol).copied() {
      let kind = variable.borrow().kind;
      if kind.is_untracked() {
        new_val.unknown_mutation(self);
      } else if kind.is_const() {
        self.thrown_builtin_error("Cannot assign to const variable");
        new_val.unknown_mutation(self);
      } else {
        let variable_ref = variable.borrow();
        let target_cf_scope = self.find_first_different_cf_scope(variable_ref.cf_scope);

        if !variable_ref.exhausted {
          let old_val = variable_ref.value;
          let (should_consume, indeterminate) = if old_val.is_some() {
            // Normal write
            self.mark_exhaustive_write((id, symbol), target_cf_scope)
          } else if !variable_ref.kind.is_redeclarable() {
            // TDZ write
            self.handle_tdz(target_cf_scope);
            (true, false)
          } else {
            // Write uninitialized `var`
            self.mark_exhaustive_write((id, symbol), target_cf_scope)
          };
          drop(variable_ref);

          let mut variable_ref = variable.borrow_mut();
          if should_consume {
            variable_ref.exhausted = true;
            variable_ref.value = Some(self.factory.unknown);
          } else {
            variable_ref.value = Some(if indeterminate {
              self.factory.union((old_val.unwrap_or(self.factory.undefined), new_val))
            } else {
              new_val
            });
          };
          drop(variable_ref);

          self.add_exhaustive_callbacks(should_consume, (id, symbol));
        }
      }
      true
    } else {
      false
    }
  }

  pub fn consume_on_scope(&mut self, id: ScopeId, symbol: SymbolId) -> bool {
    if let Some(variable) = self.scope_context.variable.get(id).variables.get(&symbol).copied() {
      let variable_ref = variable.borrow();
      if !variable_ref.exhausted {
        if let Some(value) = &variable_ref.value {
          value.unknown_mutation(self);
        }
        drop(variable_ref);

        let mut variable_ref = variable.borrow_mut();
        variable_ref.exhausted = true;
        variable_ref.value = Some(self.factory.unknown);
      }
      true
    } else {
      false
    }
  }

  fn mark_untracked_on_scope(&mut self, symbol: SymbolId) {
    let cf_scope_depth = self.call_scope().cf_scope_depth;
    let variable = self.allocator.alloc(RefCell::new(Variable {
      exhausted: true,
      kind: DeclarationKind::UntrackedVar,
      cf_scope: self.scope_context.cf.stack[cf_scope_depth],
      value: Some(self.factory.unknown),
      decl_node: AstKind2::Environment,
    }));
    let old = self.variable_scope_mut().variables.insert(symbol, variable);
    assert!(old.is_none());
  }

  pub fn consume_arguments_on_scope(&mut self, id: ScopeId) -> bool {
    if let Some((args_entity, args_symbols)) = self.scope_context.variable.get(id).arguments.clone()
    {
      args_entity.unknown_mutation(self);
      let mut arguments_consumed = true;
      for symbol in args_symbols {
        if !self.consume_on_scope(id, symbol) {
          // Still inside parameter declaration
          arguments_consumed = false;
        }
      }
      arguments_consumed
    } else {
      true
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn declare_symbol(
    &mut self,
    symbol: SymbolId,
    decl_node: AstKind2<'a>,
    exporting: bool,
    kind: DeclarationKind,
    fn_value: Option<Entity<'a>>,
  ) {
    if exporting {
      self.named_exports.push(symbol);
    }
    if kind == DeclarationKind::FunctionParameter {
      if let Some(arguments) = &mut self.variable_scope_mut().arguments {
        arguments.1.push(symbol);
      }
    }

    let variable_scope = self.scope_context.variable.current_id();
    self.declare_on_scope(variable_scope, kind, symbol, decl_node, fn_value);
  }

  pub fn init_symbol(&mut self, symbol: SymbolId, value: Option<Entity<'a>>) {
    let variable_scope = self.scope_context.variable.current_id();
    self.init_on_scope(variable_scope, symbol, value);
  }

  /// `None` for TDZ
  pub fn read_symbol(&mut self, symbol: SymbolId) -> Option<Entity<'a>> {
    for depth in (0..self.scope_context.variable.stack.len()).rev() {
      let id = self.scope_context.variable.stack[depth];
      if let Some(value) = self.read_on_scope(id, symbol) {
        return value;
      }
    }
    self.mark_unresolved_reference(symbol);
    Some(self.factory.unknown)
  }

  pub fn write_symbol(&mut self, symbol: SymbolId, new_val: Entity<'a>) {
    for depth in (0..self.scope_context.variable.stack.len()).rev() {
      let id = self.scope_context.variable.stack[depth];
      if self.write_on_scope(id, symbol, new_val) {
        return;
      }
    }
    new_val.unknown_mutation(self);
    self.mark_unresolved_reference(symbol);
  }

  fn mark_unresolved_reference(&mut self, symbol: SymbolId) {
    if self.semantic.symbols().get_flags(symbol).is_function_scoped_declaration() {
      self.mark_untracked_on_scope(symbol);
    } else {
      self.thrown_builtin_error("Unresolved identifier reference");
    }
  }

  pub fn handle_tdz(&mut self, target_cf_scope: usize) {
    if self.has_exhaustive_scope_since(target_cf_scope) {
      self.may_throw();
    } else {
      self.thrown_builtin_error("Cannot access variable before initialization");
    }
  }

  pub fn get_this(&self) -> Entity<'a> {
    for depth in (0..self.scope_context.variable.stack.len()).rev() {
      let scope = self.scope_context.variable.get_from_depth(depth);
      if let Some(this) = scope.this {
        return this;
      }
    }
    unreachable!()
  }
}
