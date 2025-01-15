use super::Ty;
use oxc::allocator::Allocator;

pub fn into_union<'a>(allocator: &'a Allocator, types: Vec<Ty<'a>>) -> Ty<'a> {
  match types.len() {
    0 => Ty::Undefined,
    1 => types.into_iter().next().unwrap(),
    _ => Ty::Union(allocator.alloc(types)),
  }
}
