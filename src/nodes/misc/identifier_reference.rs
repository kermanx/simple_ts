use oxc::ast::ast::IdentifierReference;

use crate::{analyzer::Analyzer, ty::Ty};

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
}
