use oxc::ast::ast::{Expression, TSInterfaceDeclaration};

use crate::{
  Analyzer,
  ty::{Ty, interface::InterfaceTypeInner, unresolved::UnresolvedType},
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) {
    let symbol_id = node.id.symbol_id();
    self
      .type_scopes
      .insert_on_top(symbol_id, Ty::Unresolved(UnresolvedType::UnInitType(symbol_id)));
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

    if let Some(extends) = &node.extends {
      for heritage in extends {
        match &heritage.expression {
          Expression::Identifier(id) => {
            let base = self.resolve_type_identifier_reference(id);
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
