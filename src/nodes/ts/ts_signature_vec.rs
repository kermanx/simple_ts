use crate::{
  ty::{record::RecordType, Ty},
  Analyzer,
};
use oxc::{allocator, ast::ast::TSSignature};

impl<'a> Analyzer<'a> {
  pub fn resolve_signature_vec(
    &mut self,
    node: &'a allocator::Vec<'a, TSSignature<'a>>,
    record: &mut Option<&mut RecordType<'a>>,
  ) -> Vec<Ty<'a>> {
    let allocator = self.allocator;
    let alloc_record = || allocator.alloc(RecordType::default());
    let callables = vec![];

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
        TSSignature::TSCallSignatureDeclaration(node) => todo!(),
        TSSignature::TSConstructSignatureDeclaration(node) => todo!(),
        TSSignature::TSMethodSignature(node) => todo!(),
      }
    }

    callables
  }
}
