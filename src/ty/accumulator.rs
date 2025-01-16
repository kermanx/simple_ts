use oxc::allocator::Allocator;
use std::mem;

use crate::Analyzer;

use super::{property_key::PropertyKeyType, union::UnionType, Ty};

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

  pub fn to_ty(&mut self) -> Option<Ty<'a>> {
    match &*self {
      TypeAccumulator::None => None,
      TypeAccumulator::Single(ty) => Some(*ty),
      TypeAccumulator::Union(_) => match mem::take(self) {
        TypeAccumulator::Union(union) => {
          *self = TypeAccumulator::FrozenUnion(union);
          Some(Ty::Union(union))
        }
        _ => unreachable!(),
      },
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
