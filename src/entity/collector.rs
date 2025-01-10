use super::{Entity, LiteralEntity};
use crate::analyzer::Analyzer;

#[derive(Debug, Default)]
pub struct LiteralCollector<'a> {
  /// None if no literal is collected
  literal: Option<LiteralEntity<'a>>,
  /// Collected literal entities
  collected: Vec<Entity<'a>>,
  invalid: bool,
}

impl<'a> LiteralCollector<'a> {
  pub fn collect(&mut self, analyzer: &mut Analyzer<'a>, entity: Entity<'a>) -> Entity<'a> {
    if self.invalid {
      entity
    } else if let Some(literal) = entity.get_literal(analyzer) {
      if let Some(collected) = self.literal {
        if collected == literal {
          self.on_collectable(analyzer, entity, literal)
        } else {
          self.on_invalid(analyzer, entity)
        }
      } else {
        self.literal = Some(literal);
        self.on_collectable(analyzer, entity, literal)
      }
    } else {
      self.on_invalid(analyzer, entity)
    }
  }

  fn on_invalid(&mut self, _analyzer: &mut Analyzer<'a>, entity: Entity<'a>) -> Entity<'a> {
    self.invalid = true;
    entity
  }

  fn on_collectable(
    &mut self,
    _analyzer: &mut Analyzer<'a>,
    entity: Entity<'a>,
    _literal: LiteralEntity<'a>,
  ) -> Entity<'a> {
    self.collected.push(entity);
    entity
  }

  pub fn collected(&self) -> Option<LiteralEntity<'a>> {
    if self.invalid {
      None
    } else {
      self.literal
    }
  }
}
