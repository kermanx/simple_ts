use oxc::ast::ast::MetaProperty;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_meta_property(&mut self, node: &'a MetaProperty<'a>, _sat: Option<Ty<'a>>) -> Ty<'a> {
    todo!()
  }
}
