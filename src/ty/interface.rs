use std::cell::RefCell;

use oxc::ast::ast::TSType;

use crate::{
  Analyzer,
  allocator::{self, Allocator},
};

use super::{Ty, property_key::PropertyKeyType, record::RecordType, unresolved::UnresolvedType};

#[derive(Debug)]
pub struct InterfaceTypeInner<'a> {
  pub record: RecordType<'a>,
  pub callables: allocator::Vec<'a, Ty<'a>>,
  pub unresolved_extends: allocator::Vec<'a, UnresolvedType<'a>>,
}

#[derive(Debug)]
pub struct InterfaceType<'a>(pub RefCell<InterfaceTypeInner<'a>>);

impl<'a> InterfaceTypeInner<'a> {
  pub fn extend(&mut self, ty: Ty<'a>) {
    match ty {
      Ty::Record(r) => {}
      Ty::Constructor(_) | Ty::Function(_) => self.callables.push(ty),
      Ty::Interface(i) => {
        let i = i.0.borrow();
        self.record.extend(&i.record);
        self.callables.extend(i.callables.iter().cloned());
      }
      Ty::Namespace(n) => todo!(),

      Ty::Intersection(i) => {
        i.for_each(|ty| self.extend(ty));
      }

      Ty::Unresolved(u) => {
        self.unresolved_extends.push(u);
      }

      _ => {
        // Should error
      }
    }
  }
}

impl<'a> InterfaceType<'a> {
  pub fn new_in(allocator: Allocator<'a>) -> Self {
    let inner = InterfaceTypeInner {
      record: RecordType::new_in(allocator),
      callables: allocator.vec(),
      unresolved_extends: allocator.vec(),
    };
    Self(RefCell::new(inner))
  }

  pub fn get_property(&self, key: PropertyKeyType<'a>) -> Ty<'a> {
    let inner = self.0.borrow();
    inner.record.get_property(key)
  }

  pub fn is_empty(&self) -> bool {
    let inner = self.0.borrow();
    inner.record.is_empty() && inner.callables.is_empty() && inner.unresolved_extends.is_empty()
  }
}

impl<'a> Analyzer<'a> {
  pub fn serialize_interface_type(&mut self, interface: &InterfaceType<'a>) -> TSType<'a> {
    todo!()
  }
}
