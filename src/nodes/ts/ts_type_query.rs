use oxc::ast::ast::{TSTypeQuery, TSTypeQueryExprName};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_query(&mut self, node: &'a TSTypeQuery<'a>) -> Ty<'a> {
    let base = match &node.expr_name {
      TSTypeQueryExprName::IdentifierReference(node) => {
        self.exec_identifier_reference_read(node, None)
      }
      TSTypeQueryExprName::TSImportType(_node) => todo!(),
      TSTypeQueryExprName::QualifiedName(_node) => todo!(),
    };

    if let Some(type_arguments) = &node.type_arguments {
      let type_arguments = self.resolve_type_parameter_instantiation(type_arguments);
      self.create_generic_instance(base, type_arguments)
    } else {
      base
    }
  }
}
