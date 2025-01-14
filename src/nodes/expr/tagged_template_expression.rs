use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TaggedTemplateExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_tagged_template_expression(
    &mut self,
    node: &'a TaggedTemplateExpression<'a>,
  ) -> Type<'a> {
    let (indeterminate, tag, this) = self.exec_callee(&node.tag);

    if indeterminate {
      self.pop_cf_scope();
    }

    let mut arguments = vec![todo!()];

    for expr in &node.quasi.expressions {
      let value = self.exec_expression(expr);
      arguments.push((false, value));
    }

    self.get_call_return(tag, this, todo!())
  }
}
