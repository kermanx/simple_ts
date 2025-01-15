use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::ast::ast::{ArrayExpression, ArrayExpressionElement};

impl<'a> Analyzer<'a> {
  pub fn exec_array_expression(&mut self, node: &'a ArrayExpression<'a>) -> Ty<'a> {
    let mut values = vec![];

    for element in &node.elements {
      let value = match element {
        ArrayExpressionElement::SpreadElement(node) => self.exec_spread_element(node),
        ArrayExpressionElement::Elision(_node) => Ty::Undefined,
        _ => self.exec_expression(element.to_expression()),
      };
      values.push(value);
    }

    into_union(self.allocator, values)
  }
}
