use std::cell::RefCell;

use oxc::{ast::ast::TSType, span::Atom};

use super::{Ty, record::RecordType};
use crate::{
  allocator::{self, Allocator},
  analyzer::Analyzer,
};

#[derive(Debug)]
pub struct NsInner<'a> {
  pub variables: RecordType<'a>,
  pub types: allocator::HashMap<'a, Atom<'a>, Ty<'a>>,
  pub children: allocator::HashMap<'a, Atom<'a>, &'a Ns<'a>>,
}

#[derive(Debug)]
pub struct Ns<'a>(pub RefCell<NsInner<'a>>);

impl<'a> From<NsInner<'a>> for Ns<'a> {
  fn from(inner: NsInner<'a>) -> Self {
    Self(RefCell::new(inner))
  }
}

impl<'a> Ns<'a> {
  pub fn new_in(allocator: Allocator<'a>) -> Self {
    Self(RefCell::new(NsInner {
      variables: RecordType::new_in(allocator),
      types: allocator::HashMap::new_in(allocator),
      children: allocator::HashMap::new_in(allocator),
    }))
  }

  pub fn borrow(&self) -> std::cell::Ref<'_, NsInner<'a>> {
    self.0.borrow()
  }

  pub fn borrow_mut(&self) -> std::cell::RefMut<'_, NsInner<'a>> {
    self.0.borrow_mut()
  }

  pub fn variables(&'a self) -> &'a RecordType<'a> {
    unsafe { &(&*self.0.as_ptr()).variables }
  }
}

impl<'a> Analyzer<'a> {
  pub fn serialize_namespace_type(&mut self, namespace: &Ns<'a>) -> TSType<'a> {
    todo!()
  }
}
