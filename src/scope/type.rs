use std::collections::hash_map::Entry;

use oxc::semantic::SymbolId;
use oxc_index::{define_index_type, IndexVec};
use rustc_hash::FxHashMap;

use crate::{
  ty::{unresolved::UnresolvedType, Ty},
  Analyzer,
};

define_index_type! {
  pub struct TypeScopeId = u32;
}

#[derive(Debug)]
struct TypeScope<'a> {
  types: FxHashMap<SymbolId, Ty<'a>>,
  parent: Option<TypeScopeId>,
}

#[derive(Debug)]
pub struct TypeScopeTree<'a> {
  nodes: IndexVec<TypeScopeId, TypeScope<'a>>,
  root: TypeScopeId,
  top: TypeScopeId,
}

impl<'a> TypeScopeTree<'a> {
  pub fn new() -> Self {
    let mut nodes = IndexVec::new();
    let root = nodes.push(TypeScope { types: FxHashMap::default(), parent: None });
    TypeScopeTree { nodes, root, top: root }
  }

  pub fn push(&mut self) -> TypeScopeId {
    self.push_with_types(FxHashMap::default())
  }

  pub fn push_with_types(&mut self, types: FxHashMap<SymbolId, Ty<'a>>) -> TypeScopeId {
    let id = self.nodes.push(TypeScope { types, parent: Some(self.top) });
    self.top = id;
    id
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

  pub fn insert(&mut self, id: SymbolId, ty: Ty<'a>) -> Option<Ty<'a>> {
    self.nodes[self.top].types.insert(id, ty)
  }

  pub fn entry(&mut self, id: SymbolId) -> Entry<SymbolId, Ty<'a>> {
    self.nodes[self.top].types.entry(id)
  }

  pub fn get(&self, id: SymbolId) -> Option<&Ty<'a>> {
    self.nodes[self.top].types.get(&id)
  }

  pub fn get_mut(&mut self, id: SymbolId) -> Option<&mut Ty<'a>> {
    self.nodes[self.top].types.get_mut(&id)
  }
}
