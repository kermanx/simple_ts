use crate::{builtins::Builtins, init_map};

impl<'a> Builtins<'a> {
  pub fn init_global_constants(&mut self) {
    let factory = self.factory;

    init_map!(self.globals, {
      "undefined" => factory.undefined,
      "Infinity" => factory.number,
      "NaN" => factory.number,
      "eval" => factory.unknown,
    })
  }
}
