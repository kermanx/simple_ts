pub mod call;
pub mod control;
pub mod runtime;
pub mod r#type;
pub mod variable;

use oxc::{semantic::SymbolId, span::Atom};

use crate::{
  Analyzer,
  ty::{Ty, record::RecordPropertyValue},
};

impl<'a> Analyzer<'a> {
  pub fn update_namespace(
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
      ns.record.string_keyed.0.insert(name.as_str(), property);
    }
  }
}
