use oxc::ast::ast::{JSXExpression, JSXExpressionContainer};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_expression_container_as_attribute_value(
    &mut self,
    node: &'a JSXExpressionContainer<'a>,
    ty: Option<Ty<'a>>,
  ) -> Ty<'a> {
    match &node.expression {
      JSXExpression::EmptyExpression(_node) => Ty::Boolean,
      node => self.exec_expression(node.to_expression(), ty),
    }
  }

  pub fn exec_jsx_expression_container_as_jsx_child(
    &mut self,
    node: &'a JSXExpressionContainer<'a>,
  ) {
    match &node.expression {
      JSXExpression::EmptyExpression(_node) => {}
      node => {
        self.exec_expression(node.to_expression(), None);
      }
    }
  }
}
