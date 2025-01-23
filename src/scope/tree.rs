use std::ops::RangeFrom;

use oxc::semantic::ScopeId;
use oxc_index::{Idx, IndexVec};


impl<T> Default for ScopeTree<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> ScopeTree<T> {
  pub fn new() -> Self {
    ScopeTree { nodes: IndexVec::new(), stack: vec![] }
  }

  pub fn current_id(&self) -> ScopeId {
    *self.stack.last().unwrap()
  }

  pub fn current_depth(&self) -> usize {
    self.stack.len() - 1
  }

  pub fn get(&self, id: ScopeId) -> &T {
    &self.nodes.get(id).unwrap().data
  }

  pub fn get_mut(&mut self, id: ScopeId) -> &mut T {
    &mut self.nodes.get_mut(id).unwrap().data
  }

  pub fn get_from_depth(&self, depth: usize) -> &T {
    let id = self.stack[depth];
    self.get(id)
  }

  pub fn get_mut_from_depth(&mut self, depth: usize) -> &mut T {
    let id = self.stack[depth];
    self.get_mut(id)
  }

  pub fn get_current(&self) -> &T {
    self.get(*self.stack.last().unwrap())
  }

  pub fn get_current_mut(&mut self) -> &mut T {
    self.get_mut(*self.stack.last().unwrap())
  }

  pub fn iter_stack(&self) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator<Item = &T> {
    self.stack.iter().map(move |id| self.get(*id))
  }

  pub fn iter_stack_range(
    &self,
    range: RangeFrom<usize>,
  ) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator<Item = &T> {
    self.stack[range].iter().map(move |id| self.get(*id))
  }

  pub fn iter_all(&self) -> impl Iterator<Item = &T> {
    self.nodes.iter().map(|node| &node.data)
  }

  pub fn add_special(&mut self, data: T) -> ScopeId {
    let id = ScopeId::from_usize(self.nodes.len());
    self.nodes.push(NodeInfo { data, depth: 0, parent: None });
    id
  }

  pub fn push(&mut self, data: T) -> ScopeId {
    let id = ScopeId::from_usize(self.nodes.len());
    self.nodes.push(NodeInfo { data, depth: self.stack.len(), parent: self.stack.last().copied() });
    self.stack.push(id);
    id
  }

  pub fn pop(&mut self) -> ScopeId {
    self.stack.pop().unwrap()
  }
}
