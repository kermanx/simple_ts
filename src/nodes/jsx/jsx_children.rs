use crate::{analyzer::Analyzer, entity::Entity};
use oxc::{allocator, ast::ast::JSXChild};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_children(&mut self, node: &'a allocator::Vec<'a, JSXChild<'a>>) -> Entity<'a> {
    let values: Vec<_> = node
      .iter()
      .map(|child| match child {
        JSXChild::Text(node) => self.exec_jsx_text(node),
        JSXChild::Element(node) => self.exec_jsx_element(node),
        JSXChild::Fragment(node) => self.exec_jsx_fragment(node),
        JSXChild::ExpressionContainer(node) => {
          self.exec_jsx_expression_container_as_jsx_child(node)
        }
        JSXChild::Spread(node) => self.exec_jsx_spread_child(node),
      })
      .collect();
    self.factory.unknown
  }
}
