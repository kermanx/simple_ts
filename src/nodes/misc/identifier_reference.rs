use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::IdentifierReference;

impl<'a> Analyzer<'a> {
  pub fn exec_identifier_reference_read(
    &mut self,
    node: &'a IdentifierReference<'a>,
  ) -> Entity<'a> {
    let reference = self.semantic.symbols().get_reference(node.reference_id());
    let symbol = reference.symbol_id();

    if let Some(symbol) = symbol {
      // Known symbol
      if let Some(value) = self.read_symbol(symbol) {
        value
      } else {
        // TDZ
        self.factory.unknown
      }
    } else if node.name == "arguments" {
      // The `arguments` object
      let arguments_consumed = self.consume_arguments();
      self.call_scope_mut().need_consume_arguments = !arguments_consumed;
      self.factory.unknown
    } else if let Some(global) = self.builtins.globals.get(node.name.as_str()) {
      // Known global
      *global
    } else {
      // Unknown global
      if self.is_inside_pure() {
        self.factory.unknown
      } else {
        self.factory.unknown
      }
    }
  }

  pub fn exec_identifier_reference_write(
    &mut self,
    node: &'a IdentifierReference<'a>,
    value: Entity<'a>,
  ) {
    let reference = self.semantic.symbols().get_reference(node.reference_id());
    assert!(reference.is_write());
    let symbol = reference.symbol_id();

    if let Some(symbol) = symbol {
      self.write_symbol(symbol, value);
    } else if self.builtins.globals.contains_key(node.name.as_str()) {
      self.add_diagnostic(
        "Should not write to builtin object, it may cause unexpected tree-shaking behavior",
      );
    } else {
      value.unknown_mutation(self);
      self.may_throw();
    }
  }
}
