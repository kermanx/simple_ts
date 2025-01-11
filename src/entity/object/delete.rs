use super::ObjectEntity;
use crate::{
  analyzer::Analyzer,
  entity::{consumed_object, Entity, EntityTrait, LiteralEntity},
};

impl<'a> ObjectEntity<'a> {
  pub fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    if self.consumed.get() {
      return consumed_object::delete_property(analyzer, key);
    }

    let (has_exhaustive, indeterminate) = analyzer.pre_mutate_object(self.cf_scope, self.object_id);

    if has_exhaustive {
      self.unknown_mutation(analyzer);
      return consumed_object::delete_property(analyzer, key);
    }

    let key = key.get_to_property_key(analyzer);

    {
      let mut unknown_keyed = self.unknown_keyed.borrow_mut();
      if !unknown_keyed.possible_values.is_empty() {
        unknown_keyed.delete(true);
      }
    }

    if let Some(key_literals) = key.get_to_literals(analyzer) {
      let indeterminate = indeterminate || key_literals.len() > 1;

      let mut string_keyed = self.string_keyed.borrow_mut();
      let mut rest = self.rest.borrow_mut();
      for key_literal in key_literals {
        match key_literal {
          LiteralEntity::String(key_str) => {
            if let Some(property) = string_keyed.get_mut(key_str) {
              property.delete(indeterminate);
            } else if let Some(rest) = &mut *rest {
              rest.delete(true);
            }
          }
          LiteralEntity::Symbol(_, _) => todo!(),
          _ => unreachable!("Invalid property key"),
        }
      }
    } else {
      let mut string_keyed = self.string_keyed.borrow_mut();
      for property in string_keyed.values_mut() {
        property.delete(true);
      }
    }
  }
}
