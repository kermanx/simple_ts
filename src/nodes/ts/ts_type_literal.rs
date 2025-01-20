use crate::{
  ty::{
    intersection::{IntersectionBaseKind, IntersectionType},
    record::RecordType,
    Ty,
  },
  Analyzer,
};
use oxc::ast::ast::{TSSignature, TSTypeLiteral};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_literal(&mut self, node: &'a TSTypeLiteral<'a>) -> Ty<'a> {
    let allocator = self.allocator;
    let mut record = None;
    let alloc_record = || allocator.alloc(RecordType::default());
    let mut callables = vec![];

    for member in &node.members {
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
            todo!("Wtf, how can this happen?");
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

    if callables.is_empty() {
      Ty::Record(record.unwrap_or_else(alloc_record))
    } else {
      if let Some(record) = record {
        callables.push(Ty::Record(record));
      }
      Ty::Intersection(
        allocator.alloc(IntersectionType::ObjectLike(IntersectionBaseKind::NoBase, callables)),
      )
    }
  }
}
