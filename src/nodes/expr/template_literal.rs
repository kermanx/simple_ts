use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TemplateLiteral;

impl<'a> Analyzer<'a> {
  pub fn exec_template_literal(
    &mut self,
    node: &'a TemplateLiteral<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    for expr in &node.expressions {
      self.exec_expression(expr, None);
    }
    Ty::String
  }
}
