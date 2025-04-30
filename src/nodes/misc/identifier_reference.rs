use oxc::ast::ast::IdentifierReference;

use crate::{
  analyzer::Analyzer,
  ty::{Ty, namespace::Ns},
};

impl<'a> Analyzer<'a> {
  pub fn exec_identifier_reference_read(
    &mut self,
    node: &'a IdentifierReference<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let reference = self.semantic.scoping().get_reference(node.reference_id());
    let symbol = reference.symbol_id();

    if let Some(symbol) = symbol {
      self.read_variable(symbol)
    } else {
      // TODO: globals and `arguments`
      Ty::Unknown
    }
  }

  pub fn exec_identifier_reference_write(
    &mut self,
    node: &'a IdentifierReference<'a>,
    value: Ty<'a>,
  ) {
    let reference = self.semantic.scoping().get_reference(node.reference_id());
    assert!(reference.is_write());
    let symbol = reference.symbol_id();

    if let Some(symbol) = symbol {
      self.write_variable(symbol, value);
    } else {
      // TODO: globals and `arguments`
    }
  }

  pub fn resolve_identifier_reference_ty(&mut self, node: &'a IdentifierReference<'a>) -> Ty<'a> {
    let reference = self.semantic.scoping().get_reference(node.reference_id());
    if let Some(symbol_id) = reference.symbol_id() {
      self.type_scopes.search(symbol_id)
    } else {
      // TODO: Global type
      Ty::Unknown
    }
  }

  pub fn resolve_identifier_reference_ns(
    &mut self,
    node: &'a IdentifierReference<'a>,
  ) -> Option<&'a Ns<'a>> {
    let reference = self.semantic.scoping().get_reference(node.reference_id());
    if let Some(symbol_id) = reference.symbol_id() {
      if let Some(namespace) = self.namespaces.get(&symbol_id) { Some(*namespace) } else { todo!() }
    } else {
      None
    }
  }
}
