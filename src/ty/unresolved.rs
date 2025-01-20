use super::{
  union::{into_union, UnionType},
  Ty,
};
use crate::Analyzer;
use oxc::{ast::ast::TSType, semantic::SymbolId};

#[derive(Debug, PartialEq, Eq)]
pub enum UnresolvedType<'a> {
  UnresolvedTypedVariable(SymbolId),
  /// (symbol, constraint)
  GenericParam(SymbolId, Option<Ty<'a>>),
  /// (check, extends, true, false)
  Conditional(Ty<'a>, Ty<'a>, Ty<'a>, Ty<'a>),
  Keyof(Ty<'a>),
}

impl<'a> Analyzer<'a> {
  pub fn resolve_unresolved(&mut self, ty: Ty<'a>) -> Ty<'a> {
    self.try_resolve_unresolved(ty).unwrap_or(ty)
  }

  /// Returns `None` if unchanged
  fn try_resolve_unresolved(&mut self, ty: Ty<'a>) -> Option<Ty<'a>> {
    match ty {
      Ty::Unresolved(u) => match u {
        UnresolvedType::UnresolvedTypedVariable(_) => None,
        UnresolvedType::GenericParam(symbol, _) => match *self.generics.get(symbol).unwrap() {
          Ty::Unresolved(u2) if u == u2 => None,
          ty => self.try_resolve_unresolved(ty),
        },
        UnresolvedType::Conditional(check, extends, true_ty, false_ty) => {
          let c = self.try_resolve_unresolved(*check);
          let e = self.try_resolve_unresolved(*extends);
          let t = self.try_resolve_unresolved(*true_ty);
          let f = self.try_resolve_unresolved(*false_ty);
          match (c, e, t, f) {
            (None, None, None, None) => None,
            (c, e, t, f) => {
              let c = c.unwrap_or(*check);
              let e = e.unwrap_or(*extends);
              todo!()
            }
          }
        }
        UnresolvedType::Keyof(ty) => {
          let ty = self.try_resolve_unresolved(*ty)?;
          todo!()
        }
      },

      Ty::Record(r) => todo!(),
      Ty::Function(f) => todo!(),
      Ty::Constructor(c) => todo!(),
      Ty::Namespace(n) => todo!(),

      Ty::Union(UnionType::WithUnresolved(resolved, unresolved)) => todo!(),
      Ty::Intersection(_) => todo!(),

      _ => Some(ty),
    }
  }

  /// Returns `None` if the type is singular.
  /// This function only unwrap one level of `UnresolvedType`.
  pub fn get_unresolved_base_type(&self, unresolved: &UnresolvedType<'a>) -> Option<Ty<'a>> {
    match unresolved {
      UnresolvedType::UnresolvedTypedVariable(_) => None,
      UnresolvedType::GenericParam(_, None) => None,
      UnresolvedType::GenericParam(_, Some(ty)) => Some(*ty),
      UnresolvedType::Conditional(_, _, t, f) => Some(into_union(self.allocator, [*t, *f])),
      UnresolvedType::Keyof(_) => Some(Ty::String),
    }
  }

  pub fn print_unresolved_type(&self, unresolved: &UnresolvedType<'a>) -> TSType<'a> {
    todo!()
  }
}
