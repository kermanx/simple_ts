use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::ImportExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_import_expression(&mut self, node: &'a ImportExpression<'a>) -> Type<'a> {
    let mut deps = vec![];

    deps.push(self.exec_expression(&node.source).get_to_string(self));

    for argument in &node.arguments {
      deps.push(self.exec_expression(argument));
    }

    // FIXME: if have side effects, then consume all deps

    self.factory.unknown
  }
}
