use std::{cell::RefCell, mem};

use oxc::{ast::ast::TSType, semantic::SymbolId, span::Atom};
use rustc_hash::FxHashMap;

use super::{intersection::IntersectionType, union::UnionType, Ty};
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
  pub name: &'a Atom<'a>,
  pub params: Vec<GenericParam<'a>>,
  pub body: Ty<'a>,
}

#[derive(Debug, Clone)]
pub struct GenericInstanceType<'a> {
  pub generic: Ty<'a>,
  /// Defaults should already be applied
  pub args: Vec<Ty<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn instantiate_generic_param(&mut self, params: &Vec<GenericParam<'a>>, args: &Vec<Ty<'a>>) {
    for (index, param) in params.iter().enumerate() {
      let arg = args.get(index).copied().or(param.default).unwrap_or(Ty::Error);
      self.type_scopes.insert(param.symbol_id, arg);
    }
    for param in params.iter() {
      if let Some(constraint) = param.constraint {
        // TODO: Check constraint
      }
    }
  }

  pub fn create_generic_instance(&mut self, generic: Ty<'a>, mut args: Vec<Ty<'a>>) -> Ty<'a> {
    match generic {
      Ty::Generic(generic) => {
        for g in generic.params.iter().skip(args.len()) {
          if let Some(default) = g.default {
            args.push(default);
          } else {
            args.push(Ty::Error);
          }
        }
      }
      Ty::Intrinsic(_) => {}
      _ => return Ty::Error,
    }
    Ty::Instance(self.allocator.alloc(GenericInstanceType { generic, args }))
  }

  pub fn unwrap_generic_instance(&mut self, instance: &GenericInstanceType<'a>) -> Ty<'a> {
    match instance.generic {
      Ty::Unresolved(_) => {
        unreachable!("Generic itself should always be resolved when analyzing declaration")
      }

      // instance.generic is a generic type
      Ty::Generic(generic) => {
        self.type_scopes.push();
        self.instantiate_generic_param(&generic.params, &instance.args);
        let result = self.wrap_with_ctx(generic.body);
        self.type_scopes.pop();
        result
      }
      Ty::Intrinsic(_) => todo!(),

      // instance.generic is a generic value (function or constructor or compound of them)
      _ => self.instantiate_generic_value(instance.generic, &instance.args),
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
          Some(Ty::Intersection(self.allocator.alloc(Box::new(IntersectionType {
            kind,
            object_like,
            unresolved: intersection.unresolved.clone(),
          }))))
        }
      }

      Ty::Unresolved(_) => Some(self.create_generic_instance(ty, args.clone())),

      ty => Some(ty),
    }
  }

  pub fn instantiate_generic_value(&mut self, ty: Ty<'a>, args: &Vec<Ty<'a>>) -> Ty<'a> {
    self
      .try_instantiate_generic_value(ty, args)
      .unwrap_or_else(|| Ty::Record(self.allocator.alloc(Default::default())))
  }

  pub fn print_instance_type(&mut self, instance: &GenericInstanceType<'a>) -> TSType<'a> {
    let unwrapped = self.unwrap_generic_instance(instance);
    self.print_type(unwrapped)
  }

  pub fn print_generic_type(&mut self, generic: &GenericType<'a>) -> TSType<'a> {
    todo!()
  }
}
