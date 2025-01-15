use oxc::allocator::Allocator;
use std::mem;

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
        let union = allocator.alloc(UnionType::default());
        union.add(*t);
        union.add(ty);
        *self = TypeAccumulator::Union(union);
      }
      TypeAccumulator::Union(union) => union.add(ty),
      TypeAccumulator::FrozenUnion(_) => unreachable!(),
    }
  }

  pub fn to_ty(&mut self) -> Ty<'a> {
    match &*self {
      TypeAccumulator::None => unreachable!(),
      TypeAccumulator::Single(ty) => *ty,
      TypeAccumulator::Union(_) => match mem::take(self) {
        TypeAccumulator::Union(union) => {
          *self = TypeAccumulator::FrozenUnion(union);
          Ty::Union(union)
        }
        _ => unreachable!(),
      },
      TypeAccumulator::FrozenUnion(union) => Ty::Union(*union),
    }
  }
}
