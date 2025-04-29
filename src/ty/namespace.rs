use std::cell::RefCell;

use oxc::{ast::ast::TSType, span::Atom};

use super::record::RecordType;
use crate::{
  allocator::{self, Allocator},
  analyzer::Analyzer,
};

#[derive(Debug, Clone)]
pub struct Ns<'a> {
  pub record: &'a RecordType<'a>,
  pub children: RefCell<allocator::HashMap<'a, Atom<'a>, &'a Ns<'a>>>,
}

impl<'a> Ns<'a> {
  pub fn new_in(allocator: Allocator<'a>) -> Self {
    Ns {
      record: allocator.alloc(RecordType::new_in(allocator)),
      children: RefCell::new(allocator::HashMap::new_in(allocator)),
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn serialize_namespace_type(&mut self, namespace: &Ns<'a>) -> TSType<'a> {
    todo!()
  }
}
