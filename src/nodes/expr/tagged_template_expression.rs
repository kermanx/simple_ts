use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TaggedTemplateExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_tagged_template_expression(
    &mut self,
    node: &'a TaggedTemplateExpression<'a>,
  ) -> Type<'a> {
    let (_, tag, _, this) = match self.exec_callee(&node.tag) {
      Ok(v) => v,
      Err(v) => return v,
    };

    let mut arguments = vec![(false, self.factory.unknown)];

    for expr in &node.quasi.expressions {
      let value = self.exec_expression(expr);
      arguments.push((false, value));
    }

    let value = tag.call(self, this, self.factory.arguments(arguments));

    value
  }
}
