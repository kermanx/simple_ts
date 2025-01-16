use super::{union::UnionType, Ty};
use crate::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn get_widened_type(&mut self, ty: Ty<'a>) -> Ty<'a> {
    match ty {
      Ty::Error
      | Ty::Any
      | Ty::Unknown
      | Ty::Never
      | Ty::Void
      | Ty::BigInt
      | Ty::Boolean
      | Ty::Null
      | Ty::Number
      | Ty::Object
      | Ty::String
      | Ty::Symbol
      | Ty::Undefined => ty,

      Ty::StringLiteral(_) => Ty::String,
      Ty::NumericLiteral(_) => Ty::Number,
      Ty::BigIntLiteral(_) => Ty::BigInt,
      Ty::BooleanLiteral(_) => Ty::Boolean,
      Ty::UniqueSymbol(_) => Ty::Symbol,

      Ty::Record(_) | Ty::Function(_) | Ty::Constructor(_) | Ty::Namespace(_) => ty,

      Ty::Union(u) => {
        let widened = self.allocator.alloc(UnionType::default());
        u.for_each(|ty| widened.add(self.get_widened_type(ty)));
        Ty::Union(widened)
      }
      Ty::Intersection(_) => ty,

      Ty::Generic(_) | Ty::Intrinsic(_) => unreachable!(),

      Ty::UnresolvedType(_) | Ty::UnresolvedVariable(_) => todo!(),
    }
  }
}
