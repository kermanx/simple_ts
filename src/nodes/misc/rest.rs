use crate::{analyzer::Analyzer, entity::Entity};

impl<'a> Analyzer<'a> {
  /// const { enumerated_1, enumerated_2, ...rest } = object;
  pub fn exec_object_rest(
    &mut self,
    object: Entity<'a>,
    enumerated: Vec<Entity<'a>>,
  ) -> Entity<'a> {
    let rest = self.new_empty_object(&self.builtins.prototypes.object);
    rest.init_spread(self, object);
    for key in enumerated {
      rest.delete_property(self, key);
    }

    rest
  }
}
