use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::{ArrayExpression, ArrayExpressionElement};

impl<'a> Analyzer<'a> {
  pub fn exec_array_expression(&mut self, node: &'a ArrayExpression<'a>) -> Type<'a> {
    let array = self.new_empty_array();

    let mut rest = vec![];

    for element in &node.elements {
      match element {
        ArrayExpressionElement::SpreadElement(node) => {
          if let Some(spread) = self.exec_spread_element(node) {
            rest.push(spread);
          }
        }
        ArrayExpressionElement::Elision(_node) => {
          if rest.is_empty() {
            array.push_element(self.factory.undefined);
          } else {
            rest.push(self.factory.undefined);
          }
        }
        _ => {
          let value = self.exec_expression(element.to_expression());
          if rest.is_empty() {
            array.push_element(value);
          } else {
            rest.push(value);
          }
        }
      }
    }

    if !rest.is_empty() {
      array.init_rest(self.factory.union(rest));
    }

    array
  }
}
