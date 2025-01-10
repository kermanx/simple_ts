use crate::{
  analyzer::Analyzer,
  ast::{AstKind2, DeclarationKind},
  entity::Entity,
};
use oxc::ast::ast::BindingIdentifier;

impl<'a> Analyzer<'a> {
  pub fn declare_binding_identifier(
    &mut self,
    node: &'a BindingIdentifier<'a>,
    exporting: bool,
    kind: DeclarationKind,
  ) {
    let symbol = node.symbol_id.get().unwrap();
    self.declare_symbol(symbol, AstKind2::BindingIdentifier(node), exporting, kind, None);
  }

  pub fn init_binding_identifier(
    &mut self,
    node: &'a BindingIdentifier<'a>,
    init: Option<Entity<'a>>,
  ) {
    let symbol = node.symbol_id.get().unwrap();
    self.init_symbol(symbol, init);
  }
}
