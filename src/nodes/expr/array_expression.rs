use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::{ArrayExpression, ArrayExpressionElement};

impl<'a> Analyzer<'a> {
  pub fn exec_array_expression(&mut self, node: &'a ArrayExpression<'a>) -> Type<'a> {
    let values = self.allocator.alloc(vec![]);

    for element in &node.elements {
      let value = match element {
        ArrayExpressionElement::SpreadElement(node) => self.exec_spread_element(node),
        ArrayExpressionElement::Elision(_node) => Type::Undefined,
        _ => self.exec_expression(element.to_expression()),
      };
      values.push(value);
    }

    Type::Union(values)
  }
}
