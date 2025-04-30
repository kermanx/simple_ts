use std::{
  fmt::Debug,
  hash::Hash,
  ops::{Deref, DerefMut},
  slice,
};

use oxc::{allocator, span::Atom};

#[derive(Clone, Copy)]
pub struct Allocator<'a>(&'a allocator::Allocator);

impl<'a> From<&'a allocator::Allocator> for Allocator<'a> {
  fn from(allocator: &'a allocator::Allocator) -> Self {
    Allocator(allocator)
  }
}

impl<'a> Deref for Allocator<'a> {
  type Target = &'a allocator::Allocator;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a> Allocator<'a> {
  pub fn alloc_slice<T, I>(self, iter: I) -> &'a mut [T]
  where
    I: IntoIterator<Item = T>,
  {
    let mut vec = allocator::Vec::from_iter_in(iter, self.0);
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    unsafe { slice::from_raw_parts_mut(ptr, len) }
  }

  pub fn vec<T>(self) -> Vec<'a, T> {
    Vec(allocator::Vec::new_in(self.0), self)
  }

  pub fn alloc_atom(self, s: &str) -> &'a Atom<'a> {
    let atom = &*self.0.alloc_str(s);
    self.0.alloc(Atom::from(atom))
  }
}

pub struct Vec<'a, T>(allocator::Vec<'a, T>, Allocator<'a>);

impl<T: Debug> Debug for Vec<'_, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl<'a, T> Deref for Vec<'a, T> {
  type Target = allocator::Vec<'a, T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for Vec<'_, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T: Clone> Clone for Vec<'_, T> {
  fn clone(&self) -> Self {
    Self(allocator::Vec::from_iter_in(self.0.iter().cloned(), &self.1), self.1)
  }
}

pub struct HashMap<'a, K, V>(allocator::HashMap<'a, K, V>, Allocator<'a>);

impl<K: Debug, V: Debug> Debug for HashMap<'_, K, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_map().entries(self.0.iter()).finish()
  }
}

impl<'a, K, V> Deref for HashMap<'a, K, V> {
  type Target = allocator::HashMap<'a, K, V>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<K, V> DerefMut for HashMap<'_, K, V> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<K: Clone + Eq + Hash, V: Clone> Clone for HashMap<'_, K, V> {
  fn clone(&self) -> Self {
    let mut m = allocator::HashMap::with_capacity_in(self.0.len(), &self.1);
    for (k, v) in self.0.iter() {
      m.insert(k.clone(), v.clone());
    }
    Self(m, self.1)
  }
}

impl<'a, K, V> HashMap<'a, K, V> {
  pub fn new_in(allocator: Allocator<'a>) -> Self {
    Self(allocator::HashMap::new_in(&allocator), allocator)
  }
}

pub type HashSet<'a, T> = HashMap<'a, T, ()>;

pub use oxc::allocator::Box;
