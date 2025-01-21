use oxc::ast::ast::ImportExpression;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_import_expression(
    &mut self,
    node: &'a ImportExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let source = self.exec_expression(&node.source, Some(Ty::String));

    for argument in &node.arguments {
      // FIXME: This first argument is `ImportCallOptions`
      self.exec_expression(argument, Some(Ty::Any));
    }

    todo!()
  }
}
