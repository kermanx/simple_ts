mod globals;

use crate::ty::Ty;

pub struct Builtins<'a> {
  pub string_prototype: Ty<'a>,
  pub number_prototype: Ty<'a>,
  pub bigint_prototype: Ty<'a>,
  pub boolean_prototype: Ty<'a>,
  pub object_prototype: Ty<'a>,
  pub function_prototype: Ty<'a>,
  pub array_prototype: Ty<'a>,
  pub symbol_prototype: Ty<'a>,
}

impl Default for Builtins<'_> {
  fn default() -> Self {
    Self::new()
  }
}

impl Builtins<'_> {
  // TODO: Implement this
  pub fn new() -> Self {
    Self {
      string_prototype: Ty::Any,
      number_prototype: Ty::Any,
      bigint_prototype: Ty::Any,
      boolean_prototype: Ty::Any,
      object_prototype: Ty::Any,
      function_prototype: Ty::Any,
      array_prototype: Ty::Any,
      symbol_prototype: Ty::Any,
    }
  }
}
