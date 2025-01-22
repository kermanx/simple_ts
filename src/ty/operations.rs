use oxc_syntax::operator::BinaryOperator;

use super::{generic::GenericInstanceType, intersection::IntersectionBaseKind, Ty};
use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn binary_operation(&mut self, operator: BinaryOperator, lhs: Ty<'a>, rhs: Ty<'a>) -> Ty<'a> {
    match operator {
      BinaryOperator::Equality
      | BinaryOperator::Inequality
      | BinaryOperator::StrictEquality
      | BinaryOperator::StrictInequality => Ty::Boolean,

      BinaryOperator::LessThan
      | BinaryOperator::LessEqualThan
      | BinaryOperator::GreaterThan
      | BinaryOperator::GreaterEqualThan => Ty::Boolean,

      _ => todo!(),
    }
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
        self.into_union(types)
      }
      Ty::Intersection(i) => match i.kind {
        IntersectionBaseKind::Void => Ty::Never,
        _ => ty,
      },

      Ty::Instance(i) => {
        let resolved = self.resolve_generic_instance(i);
        self.non_nullable(resolved)
      }
      Ty::Generic(_) | Ty::Intrinsic(_) => Ty::Error,

      Ty::Unresolved(_) => Ty::Instance(
        self.allocator.alloc(GenericInstanceType::new(todo!("builtins::NonNullable"), vec![ty])),
      ),

      _ => ty,
    }
  }
}
