use crate::{analyzer::Analyzer, ty::Ty};
use oxc::{allocator, ast::ast::JSXChild};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_children(&mut self, node: &'a allocator::Vec<'a, JSXChild<'a>>) -> Ty<'a> {
    for child in node.iter() {
      match child {
        JSXChild::Text(node) => self.exec_jsx_text(node),
        JSXChild::Element(node) => {
          self.exec_jsx_element(node, None);
        }
        JSXChild::Fragment(node) => {
          self.exec_jsx_fragment(node, None);
        }
        JSXChild::ExpressionContainer(node) => {
          self.exec_jsx_expression_container_as_jsx_child(node)
        }
        JSXChild::Spread(node) => self.exec_jsx_spread_child(node),
      };
    }
    todo!("JSXElement");
  }
}
