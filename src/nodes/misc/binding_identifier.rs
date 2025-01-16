use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::BindingIdentifier;

impl<'a> Analyzer<'a> {
  pub fn declare_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, typed: bool) {
    let symbol = node.symbol_id.get().unwrap();
    self.declare_variable(symbol, typed);
  }

  pub fn init_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, init: Option<Ty<'a>>) {
    let symbol = node.symbol_id.get().unwrap();
    let flags = self.semantic.symbols().get_flags(symbol);
    let init = if let Some(init) = init {
      init
    } else {
      if flags.is_function_scoped_declaration() {
        return;
      } else {
        Ty::Undefined
      }
    };
    self.init_variable(symbol, init);
  }
}
