use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TemplateLiteral;

impl<'a> Analyzer<'a> {
  pub fn exec_template_literal(&mut self, node: &'a TemplateLiteral<'a>) -> Type<'a> {
    todo!()
  }
}
