use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::IdentifierReference;

impl<'a> Analyzer<'a> {
  pub fn exec_identifier_reference_read(&mut self, node: &'a IdentifierReference<'a>) -> Type<'a> {
    let reference = self.semantic.symbols().get_reference(node.reference_id());
    let symbol = reference.symbol_id();

    if let Some(symbol) = symbol {
      self.read_variable(symbol)
    } else {
      todo!("globals and `arguments`");
    }
  }

  pub fn exec_identifier_reference_write(
    &mut self,
    node: &'a IdentifierReference<'a>,
    value: Type<'a>,
  ) {
    let reference = self.semantic.symbols().get_reference(node.reference_id());
    assert!(reference.is_write());
    let symbol = reference.symbol_id();

    if let Some(symbol) = symbol {
      self.write_variable(symbol, value);
    } else {
      todo!("globals and `arguments`");
    }
  }
}
