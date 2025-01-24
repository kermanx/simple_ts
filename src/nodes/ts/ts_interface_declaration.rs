use oxc::ast::ast::{Expression, TSInterfaceDeclaration};

use crate::{
  ty::{interface::InterfaceTypeInner, unresolved::UnresolvedType, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) {
    let symbol_id = node.id.symbol_id();
    self.types.insert(symbol_id, Ty::Unresolved(UnresolvedType::UnInitType(symbol_id)));
  }

  pub fn init_ts_interface(&mut self, node: &'a TSInterfaceDeclaration<'a>) -> Ty<'a> {
    let symbol_id = node.id.symbol_id();

    let params =
      node.type_parameters.as_ref().map(|params| self.resolve_type_parameter_declaration(params));

    let ty = self.types.get_mut(&symbol_id).unwrap();

    let mut interface = match ty {
      Ty::Interface(interface) => interface.0.borrow_mut(),
      _ => unreachable!(),
    };

    let InterfaceTypeInner { record, callables, .. } = &mut *interface;
    self.resolve_signature_vec(&node.body.body, &mut Some(record), callables);

    if let Some(extends) = &node.extends {
      for heritage in extends {
        match &heritage.expression {
          Expression::Identifier(id) => {
            let reference = self.semantic.symbols().get_reference(id.reference_id());
            let base = self.read_type(reference.symbol_id());
            let extends = if let Some(type_parameters) = &heritage.type_parameters {
              let type_parameters = self.resolve_type_parameter_instantiation(type_parameters);
              self.create_generic_instance(base, type_parameters)
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

    *self.types.get(&symbol_id).unwrap()
  }
}
