use crate::{
  analyzer::Analyzer,
  r#type::{union::into_union, Type},
};
use oxc::ast::ast::Expression;

impl<'a> Analyzer<'a> {
  pub fn exec_with_default(&mut self, default: &'a Expression<'a>, value: Type<'a>) -> Type<'a> {
    self.push_indeterminate_cf_scope();
    let default_val = self.exec_expression(default);
    self.pop_cf_scope();

    into_union(self.allocator, vec![default_val, value])
  }
}
