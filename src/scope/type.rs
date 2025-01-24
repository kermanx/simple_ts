use std::{collections::hash_map::Entry, mem};

use oxc::semantic::SymbolId;
use oxc_index::{define_index_type, IndexVec};
use rustc_hash::FxHashMap;

use crate::ty::{unresolved::UnresolvedType, Ty};

define_index_type! {
  pub struct TypeScopeId = u32;
}

#[derive(Debug, Default)]
struct TypeScope<'a> {
  types: FxHashMap<SymbolId, Ty<'a>>,
  parent: Option<TypeScopeId>,
}

#[derive(Debug)]
pub struct TypeScopeTree<'a> {
  nodes: IndexVec<TypeScopeId, TypeScope<'a>>,
  root: TypeScopeId,
  top: TypeScopeId,
  pub constraints_scope: TypeScopeId,
  pub empty_scope: TypeScopeId,
}

impl<'a> TypeScopeTree<'a> {
  pub fn new() -> Self {
    let mut nodes = IndexVec::new();
    let root = nodes.push(TypeScope::default());
    let constraints_scope = nodes.push(TypeScope::default());
    let empty_scope = nodes.push(TypeScope::default());
    TypeScopeTree { nodes, root, top: root, constraints_scope, empty_scope }
  }

  pub fn create_scope(&mut self) -> TypeScopeId {
    self.nodes.push(TypeScope { types: Default::default(), parent: None })
  }

  pub fn push(&mut self) -> TypeScopeId {
    self.push_with_types(FxHashMap::default())
  }

  pub fn push_with_types(&mut self, types: FxHashMap<SymbolId, Ty<'a>>) -> TypeScopeId {
    let id = self.nodes.push(TypeScope { types, parent: Some(self.top) });
    self.top = id;
    id
  }

  pub fn push_existing(&mut self, id: TypeScopeId) {
    self.nodes[id].parent = Some(self.top);
    self.top = id;
  }

  pub fn pop(&mut self) {
    self.top = self.nodes[self.top].parent.unwrap();
  }

  pub fn search(&self, symbol: SymbolId) -> Ty<'a> {
    let mut scope = self.top;
    loop {
      if let Some(ty) = self.nodes[scope].types.get(&symbol) {
        return *ty;
      }
      if let Some(parent) = self.nodes[scope].parent {
        scope = parent;
      } else {
        break;
      }
    }

    Ty::Unresolved(UnresolvedType::UnInitType(symbol))
  }

  pub fn insert_on_scope(
    &mut self,
    scope: TypeScopeId,
    symbol: SymbolId,
    ty: Ty<'a>,
  ) -> Option<Ty<'a>> {
    self.nodes[scope].types.insert(symbol, ty)
  }

  pub fn insert_on_top(&mut self, symbol: SymbolId, ty: Ty<'a>) -> Option<Ty<'a>> {
    self.insert_on_scope(self.top, symbol, ty)
  }

  pub fn entry_on_top(&mut self, symbol: SymbolId) -> Entry<SymbolId, Ty<'a>> {
    self.nodes[self.top].types.entry(symbol)
  }

  pub fn get_on_scope(&self, scope: TypeScopeId, symbol: SymbolId) -> Option<Ty<'a>> {
    self.nodes[scope].types.get(&symbol).copied()
  }

  pub fn get_on_top(&self, symbol: SymbolId) -> Option<Ty<'a>> {
    self.get_on_scope(self.top, symbol)
  }

  pub fn get_mut_on_top(&mut self, symbol: SymbolId) -> Option<&mut Ty<'a>> {
    self.nodes[self.top].types.get_mut(&symbol)
  }

  pub fn top(&self) -> TypeScopeId {
    self.top
  }

  pub fn replace_top(&mut self, scope: TypeScopeId) -> TypeScopeId {
    mem::replace(&mut self.top, scope)
  }

  pub fn set_parent(&mut self, scope: TypeScopeId, parent: TypeScopeId) {
    self.nodes[scope].parent = Some(parent);
  }
}
