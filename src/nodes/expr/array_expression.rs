use oxc::ast::ast::{ArrayExpression, ArrayExpressionElement};

use crate::{
  analyzer::Analyzer,
  ty::{property_key::PropertyKeyType, Ty},
};

impl<'a> Analyzer<'a> {
  pub fn exec_array_expression(
    &mut self,
    node: &'a ArrayExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let mut values = vec![];

    for (i, element) in node.elements.iter().enumerate() {
      let value = match element {
        ArrayExpressionElement::SpreadElement(node) => {
          let iterable = self.exec_expression(&node.argument, None);
          self.iterate_result_union(iterable)
        }
        ArrayExpressionElement::Elision(_node) => Ty::Undefined,
        _ => {
          let sat = sat
            .map(|sat| self.get_property(sat, PropertyKeyType::NumericLiteral((i as f64).into())));
          self.exec_expression(element.to_expression(), sat)
        }
      };
      values.push(value);
    }

    self.into_union(values).unwrap_or(Ty::Never)
  }
}
