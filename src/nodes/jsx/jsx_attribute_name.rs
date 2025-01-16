use crate::{analyzer::Analyzer, ty::property_key::PropertyKeyType};
use oxc::{allocator::Allocator, ast::ast::JSXAttributeName, span::Atom};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attribute_name(&mut self, node: &'a JSXAttributeName<'a>) -> PropertyKeyType<'a> {
    PropertyKeyType::StringLiteral(get_text(self.allocator, node))
  }
}

fn get_text<'a>(allocator: &'a Allocator, node: &'a JSXAttributeName<'a>) -> &'a Atom<'a> {
  match node {
    JSXAttributeName::Identifier(node) => &node.name,
    JSXAttributeName::NamespacedName(node) => {
      let s = allocator.alloc(format!("{}:{}", node.namespace.name, node.property.name));
      allocator.alloc(Atom::from(s.as_str()))
    }
  }
}
