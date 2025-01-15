use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TemplateLiteral;

impl<'a> Analyzer<'a> {
  pub fn exec_template_literal(&mut self, node: &'a TemplateLiteral<'a>) -> Ty<'a> {
    todo!()
  }
}
