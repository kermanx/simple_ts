use std::mem;

use crate::allocator::Allocator;

use super::{Ty, union::UnionType};

#[derive(Debug, Default)]
pub enum TypeAccumulator<'a> {
  #[default]
  None,
  Single(Ty<'a>),
  Union(&'a mut UnionType<'a>),
  FrozenUnion(&'a UnionType<'a>),
}

impl<'a> TypeAccumulator<'a> {
  pub fn add(&mut self, ty: Ty<'a>, allocator: Allocator<'a>) {
    match self {
      TypeAccumulator::None => *self = TypeAccumulator::Single(ty),
      TypeAccumulator::Single(t) => {
        if *t != ty {
          let union = allocator.alloc(UnionType::default_in(allocator));
          union.add(*t, allocator);
          union.add(ty, allocator);
          *self = TypeAccumulator::Union(union);
        }
      }
      TypeAccumulator::Union(union) => union.add(ty, allocator),
      TypeAccumulator::FrozenUnion(_) => unreachable!(),
    }
  }

  pub fn is_empty(&self) -> bool {
    matches!(self, TypeAccumulator::None)
  }

  pub fn frozen(&mut self) {
    if let TypeAccumulator::Union(_) = self {
      match mem::take(self) {
        TypeAccumulator::Union(union) => {
          *self = TypeAccumulator::FrozenUnion(union);
        }
        _ => unreachable!(),
      }
    }
  }

  pub fn to_ty(&mut self) -> Option<Ty<'a>> {
    self.frozen();
    match &*self {
      TypeAccumulator::None => None,
      TypeAccumulator::Single(ty) => Some(*ty),
      TypeAccumulator::Union(_) => unreachable!(),
      TypeAccumulator::FrozenUnion(union) => Some(Ty::Union(union)),
    }
  }

  pub fn duplicate(&mut self) -> TypeAccumulator<'a> {
    if let Some(ty) = self.to_ty() { TypeAccumulator::Single(ty) } else { TypeAccumulator::None }
  }
}
