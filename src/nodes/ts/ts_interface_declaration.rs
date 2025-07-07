use oxc::ast::ast::{Expression, TSInterfaceDeclaration};

use crate::{
  Analyzer,
  ty::{
    Ty,
    interface::{InterfaceType, InterfaceTypeInner},
  },
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) {
    self.declare_type_identifier(
      &node.id,
      false, // FIXME: export
      Ty::Interface(self.allocator.alloc(InterfaceType::new_in(self.allocator))),
    );
  }

  pub fn init_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) -> Ty<'a> {
    let symbol_id = node.id.symbol_id();

    let params =
      node.type_parameters.as_ref().map(|params| self.resolve_type_parameter_declaration(params));

    let ty = self.type_scopes.get_mut_on_top(symbol_id).unwrap();

    let mut interface = match ty {
      Ty::Interface(interface) => interface.0.borrow_mut(),
      _ => unreachable!(),
    };

    let InterfaceTypeInner { callables, .. } = &mut *interface;
    if let Some(new_record) = self.resolve_signature_vec(&node.body.body, callables) {
      interface.record.extend(&new_record);
    }

    if !node.extends.is_empty() {
      for heritage in &node.extends {
        match &heritage.expression {
          Expression::Identifier(id) => {
            let base = self.resolve_identifier_reference_ty(&id);
            let extends = if let Some(type_arguments) = &heritage.type_arguments {
              let type_arguments = self.resolve_type_parameter_instantiation(type_arguments);
              self.create_generic_instance(base, type_arguments)
            } else {
              base
            };
            interface.extend(extends);
          }
          _ => {
            // TODO: Error: An interface can only extend an identifier/qualified-name with optional type arguments.
          }
        }
      }
    }

    self.type_scopes.get_on_top(symbol_id).unwrap()
  }
}
