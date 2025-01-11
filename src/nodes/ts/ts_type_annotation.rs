use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::TSTypeAnnotation;

impl<'a> Analyzer<'a> {
  pub fn exec_ts_type_annotation(&mut self, node: &'a TSTypeAnnotation<'a>) -> Entity<'a> {
    self.exec_ts_type(&node.type_annotation)
  }
}
