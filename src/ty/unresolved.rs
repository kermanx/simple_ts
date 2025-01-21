use super::{
  generic::GenericType,
  union::{into_union, UnionType},
  Ty,
};
use crate::Analyzer;
use oxc::{ast::ast::TSType, semantic::SymbolId};

#[derive(Debug, Clone)]
pub struct UnresolvedConditionalType<'a> {
  check: Ty<'a>,
  extends: Ty<'a>,
  true_ty: Ty<'a>,
  false_ty: Ty<'a>,
}

#[derive(Debug, Clone)]
pub struct UnresolvedGenericInstantiation<'a> {
  pub generic: UnresolvedType<'a>,
  pub args: Vec<Ty<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub enum UnresolvedType<'a> {
  UnresolvedTypedVariable(SymbolId),
  TypeAlias(SymbolId),
  GenericParam(SymbolId),
  Conditional(&'a UnresolvedConditionalType<'a>),
  Keyof(&'a Ty<'a>),
  InferType(SymbolId),
  GenericInstantiation(&'a UnresolvedGenericInstantiation<'a>),
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
        UnresolvedType::TypeAlias(symbol) => match *self.types.get(&symbol).unwrap() {
          Ty::Unresolved(UnresolvedType::TypeAlias(s)) if s == symbol => None,
          ty => self.try_resolve_unresolved(ty),
        },
        UnresolvedType::GenericParam(symbol) => match *self.generics.get(&symbol).unwrap() {
          Ty::Unresolved(UnresolvedType::GenericParam(s)) if s == symbol => None,
          ty => self.try_resolve_unresolved(ty),
        },
        UnresolvedType::Conditional(cond) => {
          let c = self.try_resolve_unresolved(cond.check);
          let e = self.try_resolve_unresolved(cond.extends);
          let t = self.try_resolve_unresolved(cond.true_ty);
          let f = self.try_resolve_unresolved(cond.false_ty);
          match (c, e, t, f) {
            (None, None, None, None) => None,
            (c, e, t, f) => {
              let c = c.unwrap_or(cond.check);
              let e = e.unwrap_or(cond.extends);
              todo!()
            }
          }
        }
        UnresolvedType::Keyof(ty) => {
          let ty = self.try_resolve_unresolved(*ty)?;
          todo!()
        }
        UnresolvedType::InferType(_) => unreachable!(),
        UnresolvedType::GenericInstantiation(g) => {
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
  pub fn get_unresolved_lowest_type(&self, unresolved: UnresolvedType<'a>) -> Option<Ty<'a>> {
    match unresolved {
      UnresolvedType::UnresolvedTypedVariable(_) => None,
      UnresolvedType::TypeAlias(symbol) => match *self.types.get(&symbol).unwrap() {
        Ty::Unresolved(UnresolvedType::TypeAlias(s)) if s == symbol => None,
        ty => Some(ty),
      },
      UnresolvedType::GenericParam(symbol) => self.generic_constraints.get(&symbol).copied(),
      UnresolvedType::Conditional(cond) => {
        Some(into_union(self.allocator, [cond.true_ty, cond.false_ty]))
      }
      UnresolvedType::Keyof(_) => Some(Ty::String),
      UnresolvedType::InferType(_) => unreachable!(),
      UnresolvedType::GenericInstantiation(g) => todo!(),
    }
  }

  pub fn print_unresolved_type(&self, unresolved: UnresolvedType<'a>) -> TSType<'a> {
    todo!()
  }
}
