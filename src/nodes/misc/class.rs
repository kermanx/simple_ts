use oxc::ast::ast::Class;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_class(&mut self, node: &'a Class<'a>, _sat: Option<Ty<'a>>) -> Ty<'a> {
    todo!()
  }

  pub fn declare_class(&mut self, node: &'a Class<'a>) {
    self.declare_binding_identifier(node.id.as_ref().unwrap(), true);
  }

  pub fn init_class(&mut self, node: &'a Class<'a>) -> Ty<'a> {
    todo!()
  }
}
