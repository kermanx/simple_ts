use oxc_syntax::operator::BinaryOperator;

use super::{
  intersection::IntersectionBaseKind,
  union::into_union,
  unresolved::{UnresolvedGenericInstantiation, UnresolvedType},
  Ty,
};
use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn binary_operation(&mut self, operator: BinaryOperator, lhs: Ty<'a>, rhs: Ty<'a>) -> Ty<'a> {
    todo!()
  }

  /// A fast path for `NonNullable<T>`
  pub fn non_nullable(&mut self, ty: Ty<'a>) -> Ty<'a> {
    match ty {
      Ty::Void | Ty::Null | Ty::Undefined => Ty::Never,

      Ty::Union(u) => {
        let mut types = vec![];
        u.for_each(|t| {
          types.push(self.non_nullable(t));
        });
        into_union(self.allocator, types)
      }
      Ty::Intersection(i) => match i.kind {
        IntersectionBaseKind::Void => Ty::Never,
        _ => ty,
      },

      Ty::Unresolved(u) => Ty::Unresolved(UnresolvedType::GenericInstantiation(
        self.allocator.alloc(UnresolvedGenericInstantiation {
          generic: todo!("builtins::NonNullable"),
          args: vec![ty],
        }),
      )),

      _ => ty,
    }
  }
}
