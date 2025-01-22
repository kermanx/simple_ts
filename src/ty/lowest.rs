use crate::Analyzer;

use super::{unresolved::UnresolvedType, Ty};

impl<'a> Analyzer<'a> {
  /// Returns `None` if the type is singular.
  /// Note: This function only unwrap one level of `UnresolvedType`.
  pub fn get_lowest_type(&mut self, ty: Ty<'a>) -> Ty<'a> {
    match ty {
      Ty::Instance(i) => self.unwrap_generic_instance(i),
      Ty::Generic(g) => self.get_lowest_type(g.body),
      Ty::Intrinsic(i) => todo!(),
      Ty::Namespace(_) => Ty::Error,

      Ty::Unresolved(unresolved) => match unresolved {
        UnresolvedType::UnInitVariable(_) => Ty::Unknown,
        UnresolvedType::UnInitType(symbol) => match *self.types.get(&symbol).unwrap() {
          Ty::Unresolved(UnresolvedType::UnInitType(s)) if s == symbol => Ty::Unknown,
          ty => ty,
        },
        UnresolvedType::GenericParam(symbol) => {
          self.generic_constraints.get(&symbol).copied().unwrap_or(Ty::Unknown)
        }
        UnresolvedType::Conditional(cond) => self.into_union([cond.true_ty, cond.false_ty]),
        UnresolvedType::Keyof(_) => Ty::String,
        UnresolvedType::InferType(_) => Ty::Unknown,
      },

      ty => ty,
    }
  }
}
