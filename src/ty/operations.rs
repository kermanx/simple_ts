use oxc_syntax::operator::BinaryOperator;

use super::{facts::Facts, intersection::IntersectionBaseKind, Ty};
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

      BinaryOperator::Addition => {
        let l = self.get_facts(lhs);
        let r = self.get_facts(rhs);
        if l.contains(Facts::T_EQ_STRING) || r.contains(Facts::T_EQ_STRING) {
          Ty::String
        } else if l.contains(Facts::T_EQ_NUMBER) && r.contains(Facts::T_EQ_NUMBER) {
          Ty::Number
        } else if l.contains(Facts::T_EQ_BIGINT) && r.contains(Facts::T_EQ_BIGINT) {
          Ty::BigInt
        } else {
          Ty::Any
        }
      }
      BinaryOperator::Subtraction
      | BinaryOperator::Multiplication
      | BinaryOperator::Division
      | BinaryOperator::Remainder
      | BinaryOperator::Exponential
      | BinaryOperator::ShiftLeft
      | BinaryOperator::ShiftRight
      | BinaryOperator::ShiftRightZeroFill
      | BinaryOperator::BitwiseAnd
      | BinaryOperator::BitwiseOR
      | BinaryOperator::BitwiseXOR => {
        let l = self.get_facts(lhs);
        let r = self.get_facts(rhs);
        if l.contains(Facts::T_NE_BIGINT) && r.contains(Facts::T_NE_BIGINT) {
          Ty::Number
        } else if l.contains(Facts::T_EQ_BIGINT) && r.contains(Facts::T_EQ_BIGINT) {
          Ty::BigInt
        } else {
          Ty::Any
        }
      }

      BinaryOperator::In | BinaryOperator::Instanceof => Ty::Boolean,
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
        self.into_union(types).unwrap()
      }
      Ty::Intersection(i) => match i.kind {
        IntersectionBaseKind::Void => Ty::Never,
        _ => ty,
      },

      Ty::Instance(i) => {
        let resolved = self.unwrap_generic_instance(i);
        self.non_nullable(resolved)
      }
      Ty::Generic(_) | Ty::Intrinsic(_) => Ty::Error,

      Ty::Unresolved(_) => self.create_generic_instance(todo!("builtins::NonNullable"), vec![ty]),

      _ => ty,
    }
  }
}
