use oxc::{ast::ast::TSType, semantic::SymbolId};

use super::Ty;
use crate::Analyzer;

#[derive(Debug, Clone)]
pub struct UnresolvedConditionalType<'a> {
  pub check: Ty<'a>,
  pub extends: Ty<'a>,
  pub true_ty: Ty<'a>,
  pub false_ty: Ty<'a>,
}

#[derive(Debug, Clone)]
pub struct UnresolvedUnion<'a> {
  pub base: Ty<'a>,
  pub unresolved: Vec<UnresolvedType<'a>>,
}

#[derive(Debug, Clone)]
pub struct UnresolvedIntersection<'a> {
  pub base: Ty<'a>,
  pub unresolved: Vec<UnresolvedType<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub enum UnresolvedType<'a> {
  UnInitVariable(SymbolId),
  UnInitType(SymbolId),
  GenericParam(SymbolId),
  Conditional(&'a UnresolvedConditionalType<'a>),
  Keyof(&'a Ty<'a>),
  InferType(SymbolId),
  Union(&'a UnresolvedUnion<'a>),
  Intersection(&'a UnresolvedIntersection<'a>),
}

impl<'a> Analyzer<'a> {
  pub fn resolve_unresolved(&mut self, ty: Ty<'a>) -> Ty<'a> {
    self.try_resolve_unresolved(ty).unwrap_or(ty)
  }

  /// Returns `None` if unchanged
  pub fn try_resolve_unresolved(&mut self, ty: Ty<'a>) -> Option<Ty<'a>> {
    match ty {
      Ty::Unresolved(u) => match u {
        UnresolvedType::UnInitVariable(_) => None,
        UnresolvedType::UnInitType(symbol) => match *self.types.get(&symbol).unwrap() {
          Ty::Unresolved(UnresolvedType::UnInitType(s)) if s == symbol => None,
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
        UnresolvedType::InferType(_) => None,
        UnresolvedType::Union(u) => {
          let base = self.try_resolve_unresolved(u.base);
          let mut changed = base.is_some();
          let mut types = vec![base.unwrap_or(u.base)];
          let mut unresolved = vec![];
          for u in &u.unresolved {
            if let Some(ty) = self.try_resolve_unresolved(Ty::Unresolved(*u)) {
              types.push(ty);
              changed = true;
            } else {
              unresolved.push(*u);
            }
          }
          changed.then(|| {
            let base = self.into_union(types);
            if unresolved.is_empty() {
              base
            } else {
              Ty::Unresolved(UnresolvedType::Union(
                self.allocator.alloc(UnresolvedUnion { base, unresolved }),
              ))
            }
          })
        }
        UnresolvedType::Intersection(i) => {
          let base = self.try_resolve_unresolved(i.base);
          let mut changed = base.is_some();
          let mut types = vec![base.unwrap_or(i.base)];
          let mut unresolved = vec![];
          for u in &i.unresolved {
            if let Some(ty) = self.try_resolve_unresolved(Ty::Unresolved(*u)) {
              types.push(ty);
              changed = true;
            } else {
              unresolved.push(*u);
            }
          }
          changed.then(|| {
            let base = self.into_intersection(types);
            if unresolved.is_empty() {
              base
            } else {
              Ty::Unresolved(UnresolvedType::Intersection(
                self.allocator.alloc(UnresolvedIntersection { base, unresolved }),
              ))
            }
          })
        }
      },

      Ty::Record(r) => todo!(),
      Ty::Function(f) => todo!(),
      Ty::Constructor(c) => todo!(),
      Ty::Namespace(n) => todo!(),

      Ty::Union(_) => todo!(),
      Ty::Intersection(_) => todo!(),

      _ => Some(ty),
    }
  }

  pub fn print_unresolved_type(&self, unresolved: UnresolvedType<'a>) -> TSType<'a> {
    todo!()
  }
}
