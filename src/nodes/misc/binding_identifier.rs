use oxc::ast::ast::BindingIdentifier;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn declare_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, typed: bool) {
    let symbol = node.symbol_id.get().unwrap();
    self.declare_variable(symbol, typed);
  }

  pub fn init_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, init: Option<Ty<'a>>) {
    let symbol = node.symbol_id.get().unwrap();
    let flags = self.semantic.scoping().symbol_flags(symbol);
    let init = if let Some(init) = init {
      self.accumulate_type(node, init);
      init
    } else {
      self.accumulate_type(node, Ty::Undefined);
      if flags.is_function_scoped_declaration() {
        return;
      } else {
        Ty::Undefined
      }
    };
    self.init_variable(symbol, init);
  }
}
