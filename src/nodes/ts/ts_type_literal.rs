use oxc::ast::ast::TSTypeLiteral;

use crate::{
  Analyzer,
  ty::{
    Ty,
    intersection::{IntersectionBaseKind, IntersectionType},
    record::RecordType,
  },
};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_literal(&mut self, node: &'a TSTypeLiteral<'a>) -> Ty<'a> {
    let mut callables = Vec::new();
    let record = self.resolve_signature_vec(&node.members, &mut callables);

    if callables.is_empty() {
      Ty::Record(
        self.allocator.alloc(record.unwrap_or_else(|| RecordType::empty_in(self.allocator))),
      )
    } else {
      if let Some(record) = record {
        callables.push(Ty::Record(self.allocator.alloc(record)));
      }
      Ty::Intersection(self.allocator.alloc(IntersectionType {
        kind: IntersectionBaseKind::NoBase,
        object_like: self.allocator.alloc_slice(callables),
        unresolved: self.allocator.alloc_slice([]),
      }))
    }
  }
}
