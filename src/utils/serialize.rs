use oxc::{
  ast::ast::{BindingIdentifier, IdentifierReference},
  semantic::SymbolId,
  span::SPAN,
};

use crate::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn serialize_binding_identifier(&mut self, symbol: SymbolId) -> BindingIdentifier<'a> {
    let name = self.semantic.scoping().symbol_name(symbol);
    let node = self.ast_builder.binding_identifier(SPAN, self.ast_builder.atom(name));
    node.set_symbol_id(symbol);
    node
  }

  pub fn serialize_identifier_reference(&mut self, symbol: SymbolId) -> IdentifierReference<'a> {
    let name = self.semantic.scoping().symbol_name(symbol);
    let reference_id = self.semantic.scoping().get_resolved_reference_ids(symbol)[0];
    let node = self.ast_builder.identifier_reference(SPAN, self.ast_builder.atom(name));
    node.set_reference_id(reference_id);
    node
  }
}
