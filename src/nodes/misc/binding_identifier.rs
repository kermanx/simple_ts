use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::BindingIdentifier;

impl<'a> Analyzer<'a> {
  pub fn declare_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, typed: bool) {
    let symbol = node.symbol_id.get().unwrap();
    self.declare_variable(symbol, typed);
  }

  pub fn init_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, init: Option<Ty<'a>>) {
    let symbol = node.symbol_id.get().unwrap();
    self.init_variable(symbol, init);
  }
}
