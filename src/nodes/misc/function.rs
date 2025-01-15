use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::Function;

impl<'a> Analyzer<'a> {
  pub fn exec_function(&mut self, node: &'a Function<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn declare_function(&mut self, node: &'a Function<'a>) {
    let symbol = node.id.as_ref().unwrap().symbol_id.get().unwrap();
    let value = self.exec_function(node);

    self.declare_variable(symbol, true);
    self.init_variable(symbol, Some(value));
  }
}
