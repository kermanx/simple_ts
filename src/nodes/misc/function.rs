use crate::{analyzer::Analyzer, r#type::Type};
use oxc::{
  ast::ast::{Function, FunctionType},
  semantic::ScopeId,
};
use std::rc::Rc;

impl<'a> Analyzer<'a> {
  pub fn exec_function(&mut self, node: &'a Function<'a>) -> Type<'a> {
    todo!()
  }

  pub fn declare_function(&mut self, node: &'a Function<'a>) {
    let symbol = node.id.as_ref().unwrap().symbol_id.get().unwrap();
    let value = self.exec_function(node);

    self.declare_variable(symbol, true);
    self.init_variable(symbol, Some(value));
  }
}
