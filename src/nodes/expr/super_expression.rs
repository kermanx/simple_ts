use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::Super;

impl<'a> Analyzer<'a> {
  pub fn exec_super(&mut self, _node: &'a Super, _sat: Option<Ty<'a>>) -> Ty<'a> {
    todo!()
  }
}
