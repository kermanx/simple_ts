use std::cell::RefCell;

use oxc::semantic::SymbolId;
use rustc_hash::FxHashMap;

use super::{callable::FunctionType, record::RecordType, unresolved::UnresolvedType, Ty};

#[derive(Debug, Default)]
pub struct InterfaceTypeInner<'a> {
  pub record: RecordType<'a>,
  pub callables: Vec<Ty<'a>>,
  pub string_keyed_methods: FxHashMap<&'a str, &'a FunctionType<'a>>,
  pub symbol_keyed_methods: FxHashMap<SymbolId, &'a FunctionType<'a>>,

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
        self.string_keyed_methods.extend(&i.string_keyed_methods);
        self.symbol_keyed_methods.extend(&i.symbol_keyed_methods);
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
  pub fn is_empty(&self) -> bool {
    let inner = self.0.borrow();
    inner.record.is_empty()
      && inner.callables.is_empty()
      && inner.string_keyed_methods.is_empty()
      && inner.symbol_keyed_methods.is_empty()
      && inner.unresolved_extends.is_empty()
  }
}
