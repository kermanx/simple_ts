use std::mem;

use oxc::allocator::Allocator;

use super::{property_key::PropertyKeyType, union::UnionType, Ty};
use crate::Analyzer;

#[derive(Debug, Default)]
pub enum TypeAccumulator<'a> {
  #[default]
  None,
  Single(Ty<'a>),
  Union(&'a mut UnionType<'a>),
  FrozenUnion(&'a UnionType<'a>),
}

impl<'a> TypeAccumulator<'a> {
  pub fn add(&mut self, ty: Ty<'a>, allocator: &'a Allocator) {
    match self {
      TypeAccumulator::None => *self = TypeAccumulator::Single(ty),
      TypeAccumulator::Single(t) => {
        if *t != ty {
          let union = allocator.alloc(UnionType::default());
          union.add(*t);
          union.add(ty);
          *self = TypeAccumulator::Union(union);
        }
      }
      TypeAccumulator::Union(union) => union.add(ty),
      TypeAccumulator::FrozenUnion(_) => unreachable!(),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      TypeAccumulator::None => true,
      _ => false,
    }
  }

  pub fn frozen(&mut self) {
    match self {
      TypeAccumulator::Union(_) => match mem::take(self) {
        TypeAccumulator::Union(union) => {
          *self = TypeAccumulator::FrozenUnion(union);
        }
        _ => unreachable!(),
      },
      _ => {}
    }
  }

  pub fn frozen_clone(&mut self) -> TypeAccumulator<'a> {
    self.frozen();
    match self {
      TypeAccumulator::None => TypeAccumulator::None,
      TypeAccumulator::Single(ty) => TypeAccumulator::Single(*ty),
      TypeAccumulator::Union(_) => unreachable!(),
      TypeAccumulator::FrozenUnion(union) => TypeAccumulator::FrozenUnion(union),
    }
  }

  pub fn to_ty(&mut self) -> Option<Ty<'a>> {
    self.frozen();
    match &*self {
      TypeAccumulator::None => None,
      TypeAccumulator::Single(ty) => Some(*ty),
      TypeAccumulator::Union(_) => unreachable!(),
      TypeAccumulator::FrozenUnion(union) => Some(Ty::Union(*union)),
    }
  }

  pub fn get_property(&self, analyzer: &mut Analyzer<'a>, key: PropertyKeyType<'a>) -> Ty<'a> {
    match self {
      TypeAccumulator::None => Ty::Error,
      TypeAccumulator::Single(ty) => analyzer.get_property(*ty, key),
      TypeAccumulator::Union(union) => analyzer.get_union_property(union, key),
      TypeAccumulator::FrozenUnion(union) => analyzer.get_union_property(union, key),
    }
  }
}
