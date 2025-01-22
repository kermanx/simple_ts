use oxc::ast::ast::TSTypeLiteral;

use crate::{
  ty::{
    intersection::{IntersectionBaseKind, IntersectionType},
    record::RecordType,
    Ty,
  },
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_literal(&mut self, node: &'a TSTypeLiteral<'a>) -> Ty<'a> {
    let mut record = None;
    let mut callables = Vec::new();
    self.resolve_signature_vec(&node.members, &mut record, &mut callables);

    if callables.is_empty() {
      Ty::Record(record.unwrap_or_else(|| self.allocator.alloc(RecordType::default())))
    } else {
      if let Some(record) = record {
        callables.push(Ty::Record(record));
      }
      Ty::Intersection(self.allocator.alloc(IntersectionType {
        kind: IntersectionBaseKind::NoBase,
        object_like: callables,
        unresolved: Vec::new(),
      }))
    }
  }
}
