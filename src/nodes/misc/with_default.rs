use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::Expression;

impl<'a> Analyzer<'a> {
  pub fn exec_with_default(
    &mut self,
    default: &'a Expression<'a>,
    value: Entity<'a>,
  ) -> (bool, Entity<'a>) {
    let is_undefined = value.test_is_undefined();

    let binding_val = match is_undefined {
      Some(true) => self.exec_expression(default),
      Some(false) => value,
      None => {
        self.push_indeterminate_cf_scope();
        let default_val = self.exec_expression(default);
        let value = self.factory.union((default_val, value));
        self.pop_cf_scope();
        value
      }
    };

    (is_undefined != Some(false), binding_val)
  }
}
