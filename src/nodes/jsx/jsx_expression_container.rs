use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::{JSXExpression, JSXExpressionContainer};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_expression_container_as_attribute_value(
    &mut self,
    node: &'a JSXExpressionContainer<'a>,
  ) -> Ty<'a> {
    match &node.expression {
      JSXExpression::EmptyExpression(_node) => Ty::Boolean,
      node => self.exec_expression(node.to_expression()),
    }
  }

  pub fn exec_jsx_expression_container_as_jsx_child(
    &mut self,
    node: &'a JSXExpressionContainer<'a>,
  ) {
    match &node.expression {
      JSXExpression::EmptyExpression(_node) => {}
      node => {
        self.exec_expression(node.to_expression());
      }
    }
  }
}
