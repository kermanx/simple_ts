use std::mem;

use oxc::allocator::Allocator;

use super::{union::UnionType, Ty};

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

  pub fn to_ty(&mut self) -> Option<Ty<'a>> {
    self.frozen();
    match &*self {
      TypeAccumulator::None => None,
      TypeAccumulator::Single(ty) => Some(*ty),
      TypeAccumulator::Union(_) => unreachable!(),
      TypeAccumulator::FrozenUnion(union) => Some(Ty::Union(*union)),
    }
  }

  pub fn duplicate(&mut self) -> TypeAccumulator<'a> {
    if let Some(ty) = self.to_ty() {
      TypeAccumulator::Single(ty)
    } else {
      TypeAccumulator::None
    }
  }
}
