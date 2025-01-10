use crate::{
  analyzer::Analyzer,
  ast::AstKind2,
  entity::{Entity, LiteralCollector},
};
use oxc::ast::ast::{JSXExpression, JSXExpressionContainer};

#[derive(Default)]
struct AsJsxChildData<'a> {
  collector: LiteralCollector<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_expression_container_as_attribute_value(
    &mut self,
    node: &'a JSXExpressionContainer<'a>,
  ) -> Entity<'a> {
    match &node.expression {
      JSXExpression::EmptyExpression(_node) => self.factory.r#true,
      node => self.exec_expression(node.to_expression()),
    }
  }

  pub fn exec_jsx_expression_container_as_jsx_child(
    &mut self,
    node: &'a JSXExpressionContainer<'a>,
  ) -> Entity<'a> {
    let data = self.load_data::<AsJsxChildData>(AstKind2::JsxExpressionContainer(node));

    let value = match &node.expression {
      JSXExpression::EmptyExpression(_node) => self.factory.string(""),
      node => self.exec_expression(node.to_expression()).get_to_jsx_child(self),
    };

    data.collector.collect(self, value)
  }
}
