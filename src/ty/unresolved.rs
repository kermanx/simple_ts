use super::{
  generic::GenericType,
  intersection::{into_intersection, IntersectionType},
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
  GenericInstantiation(&'a UnresolvedGenericInstantiation<'a>),
  Union(&'a UnresolvedUnion<'a>),
  Intersection(&'a UnresolvedIntersection<'a>),
}

impl<'a> Analyzer<'a> {
  pub fn resolve_unresolved(&mut self, ty: Ty<'a>) -> Ty<'a> {
    self.try_resolve_unresolved(ty).unwrap_or(ty)
  }

  /// Returns `None` if unchanged
  fn try_resolve_unresolved(&mut self, ty: Ty<'a>) -> Option<Ty<'a>> {
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
        UnresolvedType::GenericInstantiation(g) => {
          todo!()
        }
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
            let base = into_union(self.allocator, types);
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
            let base = into_intersection(self.allocator, types);
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

  /// Returns `None` if the type is singular.
  /// This function only unwrap one level of `UnresolvedType`.
  pub fn get_unresolved_lowest_type(&self, unresolved: UnresolvedType<'a>) -> Option<Ty<'a>> {
    match unresolved {
      UnresolvedType::UnInitVariable(_) => None,
      UnresolvedType::UnInitType(symbol) => match *self.types.get(&symbol).unwrap() {
        Ty::Unresolved(UnresolvedType::UnInitType(s)) if s == symbol => None,
        ty => Some(ty),
      },
      UnresolvedType::GenericParam(symbol) => self.generic_constraints.get(&symbol).copied(),
      UnresolvedType::Conditional(cond) => {
        Some(into_union(self.allocator, [cond.true_ty, cond.false_ty]))
      }
      UnresolvedType::Keyof(_) => Some(Ty::String),
      UnresolvedType::InferType(_) => None,
      UnresolvedType::GenericInstantiation(g) => todo!(),
      UnresolvedType::Union(_) => None,
      UnresolvedType::Intersection(i) => Some(i.base),
    }
  }

  pub fn print_unresolved_type(&self, unresolved: UnresolvedType<'a>) -> TSType<'a> {
    todo!()
  }
}
