use oxc::ast::ast::Expression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_with_default(&mut self, default: &'a Expression<'a>, init: Option<Ty<'a>>) -> Ty<'a> {
    self.push_indeterminate_scope();
    let default_val = self.exec_expression(default, None);
    self.pop_scope();

    if let Some(init) = init {
      self.into_union([default_val, init]).unwrap()
    } else {
      default_val
    }
  }
}
