use oxc::ast::ast::BindingIdentifier;

use crate::{
  analyzer::Analyzer,
  ty::{Ty, namespace::Ns, unresolved::UnresolvedType},
};

impl<'a> Analyzer<'a> {
  pub fn declare_binding_identifier(&mut self, node: &'a BindingIdentifier<'a>, typed: bool) {
    let symbol = node.symbol_id.get().unwrap();
    self.declare_variable(symbol, typed);
    self.update_namespace_variable(
      symbol,
      false, // FIXME: export
      node.name,
      Ty::Unresolved(UnresolvedType::UnInitVariable(symbol)),
    );
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
    self.update_namespace_variable(symbol, false, node.name, init);
  }

  pub fn declare_type_identifier(
    &mut self,
    node: &'a BindingIdentifier<'a>,
    export: bool,
    ty: Ty<'a>,
  ) {
    self.accumulate_type(node, ty);
    self.type_scopes.insert_on_top(node.symbol_id(), ty);
    self.update_namespace_type(export, node.name, ty);
  }

  pub fn declare_namespace_identifier(
    &mut self,
    node: &'a BindingIdentifier<'a>,
    export: bool,
    ns: &'a Ns<'a>,
  ) {
    self.namespaces.insert(node.symbol_id(), ns);
    self.update_namespace_child(export, node.name, ns);
  }
}
