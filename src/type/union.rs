use super::Type;
use oxc::allocator::Allocator;

pub fn into_union<'a>(allocator: &'a Allocator, types: Vec<Type<'a>>) -> Type<'a> {
  match types.len() {
    0 => Type::Undefined,
    1 => types.into_iter().next().unwrap(),
    _ => Type::Union(allocator.alloc(types)),
  }
}
