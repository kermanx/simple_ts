use oxc::{allocator, ast::ast::TSSignature};

use crate::{
  ty::{callable::CallableType, property_key::PropertyKeyType, record::RecordType, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn resolve_signature_vec(
    &mut self,
    node: &'a allocator::Vec<'a, TSSignature<'a>>,
    record: &mut Option<&mut RecordType<'a>>,
    callables: &mut Vec<Ty<'a>>,
  ) {
    let allocator = self.allocator;
    let alloc_record = || allocator.alloc(RecordType::default());

    for member in node {
      match member {
        TSSignature::TSIndexSignature(node) => {
          let key = self.resolve_type_annotation(&node.parameters[0].type_annotation);
          let key = self.to_property_key(key);
          let value = self.resolve_type_annotation(&node.type_annotation);
          record.get_or_insert_with(alloc_record).init_property(
            self,
            key,
            value,
            false,
            node.readonly,
          );
        }
        TSSignature::TSPropertySignature(node) => {
          let key = self.exec_property_key(&node.key);
          let value = if let Some(type_annotation) = &node.type_annotation {
            self.resolve_type_annotation(type_annotation)
          } else {
            Ty::Error
          };
          record.get_or_insert_with(alloc_record).init_property(
            self,
            key,
            value,
            node.optional,
            node.readonly,
          );
        }
        TSSignature::TSCallSignatureDeclaration(node) => {
          let type_params = node
            .type_parameters
            .as_ref()
            .map(|type_params| self.resolve_type_parameter_declaration(type_params))
            .unwrap_or_default();
          let this_param =
            node.this_param.as_ref().map(|n| self.ctx_ty_from_annotation(&n.type_annotation, None));
          let (_, params, rest_param) = self.resolve_formal_parameters(&node.params);
          let return_type = self.ctx_ty_from_annotation(&node.return_type, None);

          callables.push(Ty::Function(self.allocator.alloc(CallableType {
            bivariant: false,
            type_params,
            this_param,
            params,
            rest_param,
            return_type,
          })))
        }
        TSSignature::TSConstructSignatureDeclaration(node) => todo!(),
        TSSignature::TSMethodSignature(node) => {
          let type_params = node
            .type_parameters
            .as_ref()
            .map(|type_params| self.resolve_type_parameter_declaration(type_params))
            .unwrap_or_default();
          let this_param =
            node.this_param.as_ref().map(|n| self.ctx_ty_from_annotation(&n.type_annotation, None));
          let (_, params, rest_param) = self.resolve_formal_parameters(&node.params);
          let return_type = self.ctx_ty_from_annotation(&node.return_type, None);

          let function = Ty::Function(self.allocator.alloc(CallableType {
            bivariant: true,
            type_params,
            this_param,
            params,
            rest_param,
            return_type,
          }));

          let key = self.exec_property_key(&node.key);

          if matches!(
            key,
            PropertyKeyType::NumericLiteral(_)
              | PropertyKeyType::StringLiteral(_)
              | PropertyKeyType::UniqueSymbol(_)
          ) {
            record.get_or_insert_with(alloc_record).init_property(
              self,
              key,
              function,
              node.optional,
              false,
            );
          } else {
            // A computed property name in an interface must refer to an expression whose type is a literal type or a 'unique symbol' type.
          }
        }
      }
    }
  }
}
