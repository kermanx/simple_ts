use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::Class;

impl<'a> Analyzer<'a> {
  pub fn exec_class(&mut self, node: &'a Class<'a>) -> Ty<'a> {
    todo!()
  }

  pub fn declare_class(&mut self, node: &'a Class<'a>) {
    self.declare_binding_identifier(node.id.as_ref().unwrap(), true);
  }

  pub fn init_class(&mut self, node: &'a Class<'a>) -> Ty<'a> {
    todo!()
  }
}
