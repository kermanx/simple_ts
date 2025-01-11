use crate::config::Config;

use super::{
  arguments::ArgumentsEntity, Entity, LiteralEntity, PrimitiveEntity, PureBuiltinFnEntity,
  UnknownEntity,
};
use oxc::allocator::Allocator;
use std::cell::Cell;

pub struct EntityFactory<'a> {
  pub allocator: &'a Allocator,

  pub any: Entity<'a>,
  pub bigint: Entity<'a>,
  pub boolean: Entity<'a>,
  pub never: Entity<'a>,
  pub null: Entity<'a>,
  pub number: Entity<'a>,
  pub object: Entity<'a>,
  pub string: Entity<'a>,
  pub symbol: Entity<'a>,
  pub undefined: Entity<'a>,
  pub unknown: Entity<'a>,
  pub void: Entity<'a>,

  pub true_literal: Entity<'a>,
  pub false_literal: Entity<'a>,

  pub empty_arguments: Entity<'a>,
}

impl<'a> EntityFactory<'a> {
  pub fn new(allocator: &'a Allocator, config: &'a Config) -> EntityFactory<'a> {
    EntityFactory {
      any: allocator.alloc(PrimitiveEntity::Any),
      bigint: allocator.alloc(PrimitiveEntity::BigInt),
      boolean: allocator.alloc(PrimitiveEntity::Boolean),
      never: allocator.alloc(PrimitiveEntity::Never),
      null: allocator.alloc(PrimitiveEntity::Null),
      number: allocator.alloc(PrimitiveEntity::Number),
      object: allocator.alloc(PrimitiveEntity::Object),
      string: allocator.alloc(PrimitiveEntity::String),
      symbol: allocator.alloc(PrimitiveEntity::Symbol),
      undefined: allocator.alloc(PrimitiveEntity::Undefined),
      unknown: allocator.alloc(PrimitiveEntity::Unknown),
      void: allocator.alloc(PrimitiveEntity::Void),

      true_literal: allocator.alloc(LiteralEntity::Boolean(true)),
      false_literal: allocator.alloc(LiteralEntity::Boolean(false)),

      empty_arguments: allocator.alloc(ArgumentsEntity::default()),

      allocator,
    }
  }

  pub fn alloc<T>(&self, val: T) -> &'a mut T {
    self.allocator.alloc(val)
  }
}
