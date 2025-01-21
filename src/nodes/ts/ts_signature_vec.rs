use oxc::{allocator, ast::ast::TSSignature};

use crate::{
  ty::{callable::CallableType, record::RecordType, Ty},
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
            node.this_param.as_ref().map(|this_param| self.resovle_this_parameter(this_param));
          let (_, params, rest_param) = self.exec_formal_parameters(&node.params);
          let return_type = node
            .return_type
            .as_ref()
            .map_or(Ty::Any, |return_type| self.resolve_type_annotation(return_type));

          callables.push(Ty::Function(self.allocator.alloc(CallableType {
            type_params,
            this_param,
            params,
            rest_param,
            return_type,
          })))
        }
        TSSignature::TSConstructSignatureDeclaration(node) => todo!(),
        TSSignature::TSMethodSignature(node) => todo!(),
      }
    }
  }
}
