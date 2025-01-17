use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TaggedTemplateExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_tagged_template_expression(
    &mut self,
    node: &'a TaggedTemplateExpression<'a>,
  ) -> Ty<'a> {
    let (indeterminate, tag, this) = self.exec_callee(&node.tag);

    if indeterminate {
      self.pop_scope();
    }

    let mut arguments = vec![todo!()];

    for expr in &node.quasi.expressions {
      let value = self.exec_expression(expr);
      arguments.push((false, value));
    }

    todo!()
  }
}
