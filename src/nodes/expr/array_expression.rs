use oxc::ast::ast::{ArrayExpression, ArrayExpressionElement};

use crate::{
  analyzer::Analyzer,
  ty::{
    Ty,
    property_key::PropertyKeyType,
    tuple::{TupleElement, TupleType},
  },
};

impl<'a> Analyzer<'a> {
  pub fn exec_array_expression(
    &mut self,
    node: &'a ArrayExpression<'a>,
    sat: Option<Ty<'a>>,
    as_const: bool,
  ) -> Ty<'a> {
    let mut values = vec![];

    for (i, element) in node.elements.iter().enumerate() {
      let value = match element {
        ArrayExpressionElement::SpreadElement(node) => {
          (true, self.exec_expression_with_as_const(&node.argument, None, as_const))
        }
        ArrayExpressionElement::Elision(_node) => (false, Ty::Undefined),
        _ => {
          let sat = sat
            .map(|sat| self.get_property(sat, PropertyKeyType::NumericLiteral((i as f64).into())));
          (false, self.exec_expression_with_as_const(element.to_expression(), sat, as_const))
        }
      };
      values.push(value);
    }

    if as_const {
      Ty::Tuple(self.allocator.alloc(TupleType {
        elements: self.allocator.alloc_slice(values.into_iter().map(|(spread, ty)| TupleElement {
          name: None,
          spread,
          ty,
          optional: false,
        })),
        readonly: true,
      }))
    } else {
      let types = values
        .into_iter()
        .map(|(spread, ty)| if spread { self.iterate_result_union(ty) } else { ty })
        .collect::<Vec<_>>();
      let el_type = self.into_union(types).unwrap_or(Ty::Never);
      todo!("Array<{:?}>", el_type);
    }
  }
}
