use std::cell::RefCell;

use oxc::ast::ast::TSType;

use crate::Analyzer;

use super::{property_key::PropertyKeyType, record::RecordType, unresolved::UnresolvedType, Ty};

#[derive(Debug, Default)]
pub struct InterfaceTypeInner<'a> {
  pub record: RecordType<'a>,
  pub callables: Vec<Ty<'a>>,
  pub unresolved_extends: Vec<UnresolvedType<'a>>,
}

#[derive(Debug, Default)]
pub struct InterfaceType<'a>(pub RefCell<InterfaceTypeInner<'a>>);

impl<'a> InterfaceTypeInner<'a> {
  pub fn extend(&mut self, ty: Ty<'a>) {
    match ty {
      Ty::Record(r) => {}
      Ty::Constructor(_) | Ty::Function(_) => self.callables.push(ty.clone()),
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
