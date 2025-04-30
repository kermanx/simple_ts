use oxc::{semantic::SymbolId, span::Atom};

use crate::{
  Analyzer,
  ty::{Ty, namespace::Ns, record::RecordPropertyValue},
};

impl<'a> Analyzer<'a> {
  pub fn update_namespace_variable(
    &mut self,
    symbol: SymbolId,
    export: bool,
    name: Atom<'a>,
    value: Ty<'a>,
  ) {
    if export || self.declare_scopes > 0 {
      let readonly = self.semantic.scoping().symbol_flags(symbol).is_const_variable();
      let property = RecordPropertyValue { value, optional: false, readonly };
      let mut ns = self.active_namespaces.last_mut().unwrap().borrow_mut();
      ns.variables.string_keyed.0.insert(name.as_str(), property);
    }
  }

  pub fn update_namespace_type(&mut self, export: bool, name: Atom<'a>, ty: Ty<'a>) {
    if export || self.declare_scopes > 0 {
      let mut ns = self.active_namespaces.last_mut().unwrap().borrow_mut();
      ns.types.insert(name, ty);
    }
  }

  pub fn update_namespace_child(&mut self, export: bool, name: Atom<'a>, child: &'a Ns<'a>) {
    if export || self.declare_scopes > 0 {
      let mut ns = self.active_namespaces.last_mut().unwrap().borrow_mut();
      ns.children.insert(name, child);
    }
  }
}
