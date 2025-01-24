use crate::Analyzer;

use super::{unresolved::UnresolvedType, Ty};

impl<'a> Analyzer<'a> {
  /// Returns `None` if the type is singular.
  /// Note: This function only unwrap one level of `UnresolvedType`.
  pub fn get_lowest_type(&mut self, ty: Ty<'a>) -> Ty<'a> {
    match ty {
      Ty::Instance(i) => self.unwrap_generic_instance(i),
      Ty::Generic(g) => todo!(),
      Ty::Intrinsic(i) => todo!(),
      Ty::Namespace(_) => Ty::Error,

      Ty::Unresolved(unresolved) => match unresolved {
        UnresolvedType::UnInitVariable(_) => Ty::Unknown,
        UnresolvedType::UnInitType(symbol) => match self.type_scopes.search(symbol) {
          Ty::Unresolved(UnresolvedType::UnInitType(s)) if s == symbol => Ty::Unknown,
          ty => ty,
        },
        UnresolvedType::GenericParam(symbol) => {
          // if let Some(constraint) =
          //   self.type_scopes.get_on_scope(self.type_scopes.generic_constraints, symbol)
          // {
          //   self.resolve_ctx_ty(self.type_scopes.generic_constraints, *constraint)
          // } else {
          //   Ty::Unknown
          // }
          todo!()
        }
        UnresolvedType::Keyof(_) => Ty::String,
        UnresolvedType::InferType(_) => Ty::Unknown,
      },

      ty => ty,
    }
  }
}
