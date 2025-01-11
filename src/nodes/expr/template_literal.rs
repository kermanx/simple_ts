use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TemplateLiteral;

impl<'a> Analyzer<'a> {
  pub fn exec_template_literal(&mut self, node: &'a TemplateLiteral<'a>) -> Type<'a> {
    let mut result = self.factory.string_literal(node.quasi().unwrap().as_str());
    for (index, expression) in node.expressions.iter().enumerate() {
      let expression = self.exec_expression(expression);
      let quasi = self
        .factory
        .string_literal(node.quasis.get(index + 1).unwrap().value.cooked.as_ref().unwrap());
      result = self.entity_op.add(self, result, expression);
      result = self.entity_op.add(self, result, quasi);
    }
    result
  }
}
