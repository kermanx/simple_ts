use crate::{
  ty::{generic::GenericType, interface::InterfaceType, unresolved::UnresolvedType, Ty},
  Analyzer,
};
use oxc::ast::ast::{TSInterfaceDeclaration, TSType};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) {
    let symbol_id = node.id.symbol_id();
    self.types.insert(symbol_id, Ty::Unresolved(UnresolvedType::UnInitType(symbol_id)));
  }

  pub fn init_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) {
    let symbol_id = node.id.symbol_id();

    let params =
      node.type_parameters.as_ref().map(|params| self.resolve_type_parameter_declaration(params));

    let ty = self.types.get_mut(&symbol_id).unwrap();

    let interface = match ty {
      Ty::Unresolved(UnresolvedType::UnInitType(_)) => {
        let interface = &*self.allocator.alloc(InterfaceType::default());
        *ty = if let Some(params) = params {
          Ty::Generic(self.allocator.alloc(GenericType { params, body: Ty::Interface(interface) }))
        } else {
          Ty::Interface(interface)
        };
        interface
      }
      Ty::Interface(interface) => *interface,
      Ty::Generic(g) => match &g.body {
        Ty::Interface(interface) => *interface,
        _ => unreachable!(),
      },
      _ => unreachable!(),
    };

    self.resolve_signature_vec(
      &node.body.body,
      &mut Some(&mut *interface.record.borrow_mut()),
      &mut *interface.callables.borrow_mut(),
    );

    if let Some(extends) = &node.extends {
      todo!()
    }
  }

  pub fn print_interface_type(&self, interface: &InterfaceType<'a>) -> TSType<'a> {
    todo!()
  }
}
