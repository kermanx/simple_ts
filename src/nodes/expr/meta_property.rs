use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::MetaProperty;

impl<'a> Analyzer<'a> {
  pub fn exec_meta_property(&mut self, node: &'a MetaProperty<'a>) -> Type<'a> {
    todo!()
  }
}
