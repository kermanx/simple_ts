use oxc::ast::ast::Super;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_super(&mut self, _node: &'a Super, _sat: Option<Ty<'a>>) -> Ty<'a> {
    todo!()
  }
}
