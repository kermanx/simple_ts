use std::mem;

use oxc::{ast::ast::TSType, semantic::SymbolId};
use rustc_hash::FxHashMap;

use super::{
  intersection::{self, IntersectionType},
  union::UnionType,
  unresolved::{UnresolvedGenericInstantiation, UnresolvedType},
  Ty,
};
use crate::analyzer::Analyzer;

#[derive(Debug, Clone)]
pub struct GenericParam<'a> {
  pub symbol_id: SymbolId,
  pub constraint: Option<Ty<'a>>,
  pub default: Option<Ty<'a>>,
  pub r#in: bool,
  pub out: bool,
  pub r#const: bool,
}

#[derive(Debug, Clone)]
pub struct GenericType<'a> {
  pub params: Vec<GenericParam<'a>>,
  pub body: Ty<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn take_generics(&mut self) -> Box<FxHashMap<SymbolId, Ty<'a>>> {
    mem::take(&mut self.generics)
  }

  pub fn restore_generics(&mut self, old_generics: Box<FxHashMap<SymbolId, Ty<'a>>>) {
    self.generics = old_generics;
  }

  pub fn instantiate_generic_param(&mut self, params: &Vec<GenericParam<'a>>, args: &Vec<Ty<'a>>) {
    for (index, param) in params.iter().enumerate() {
      let arg = args.get(index).copied().or(param.default).unwrap_or(Ty::Error);
      self.generics.insert(param.symbol_id, arg);
    }
    for param in params.iter() {
      if let Some(constraint) = param.constraint {
        // TODO: Check constraint
      }
    }
  }

  pub fn instantiate_generic_type(&mut self, ty: Ty<'a>, args: Vec<Ty<'a>>) -> Ty<'a> {
    match ty {
      Ty::Generic(generic) => {
        let old_generics = self.take_generics();
        self.instantiate_generic_param(&generic.params, &args);
        let result = self.resolve_unresolved(generic.body);
        self.restore_generics(old_generics);
        result
      }
      Ty::Intrinsic(intrinsic) => todo!(),
      Ty::Unresolved(generic) => Ty::Unresolved(UnresolvedType::GenericInstantiation(
        self.allocator.alloc(UnresolvedGenericInstantiation { generic, args }),
      )),
      _ => unreachable!("Cannot instantiate non-generic type"),
    }
  }

  /// Returns `None` if the type parameters of callable unmatch the length of args.
  /// Example:
  /// ```ts
  /// declare const f: (() => string) & (<T>() => T);
  /// const g = f<number>; // Will be `() => number`
  /// ```
  fn try_instantiate_generic_value(&mut self, ty: Ty<'a>, args: &Vec<Ty<'a>>) -> Option<Ty<'a>> {
    match ty {
      Ty::Function(f) => self.instantiate_callable_type_parameters(f, args).map(Ty::Function),
      Ty::Constructor(c) => self.instantiate_callable_type_parameters(c, args).map(Ty::Constructor),

      Ty::Union(union) => {
        let complex =
          union.complex.iter().map(|ty| self.instantiate_generic_value(*ty, args)).collect();
        Some(Ty::Union(self.allocator.alloc(Box::new(UnionType { complex, ..union.clone() }))))
      }
      Ty::Intersection(intersection) => {
        let kind = intersection.kind;
        let object_like: Vec<_> = intersection
          .object_like
          .iter()
          .filter_map(|ty| self.try_instantiate_generic_value(*ty, args))
          .collect();
        if object_like.is_empty() {
          intersection.kind_to_ty()
        } else {
          Some(Ty::Intersection(
            self.allocator.alloc(Box::new(IntersectionType { kind, object_like })),
          ))
        }
      }

      Ty::Unresolved(u) => Some(Ty::Unresolved(UnresolvedType::GenericInstantiation(
        self.allocator.alloc(UnresolvedGenericInstantiation { generic: u, args: args.clone() }),
      ))),

      ty => Some(ty),
    }
  }

  pub fn instantiate_generic_value(&mut self, ty: Ty<'a>, args: &Vec<Ty<'a>>) -> Ty<'a> {
    self
      .try_instantiate_generic_value(ty, args)
      .unwrap_or_else(|| Ty::Record(self.allocator.alloc(Default::default())))
  }

  pub fn print_generic_type(&self, generic: &GenericType<'a>) -> TSType<'a> {
    todo!()
  }
}
