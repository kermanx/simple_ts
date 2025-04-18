use oxc::{ast::ast::JSXAttributeName, span::Atom};

use crate::{allocator::Allocator, analyzer::Analyzer, ty::property_key::PropertyKeyType};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attribute_name(&mut self, node: &'a JSXAttributeName<'a>) -> PropertyKeyType<'a> {
    PropertyKeyType::StringLiteral(get_text(self.allocator, node))
  }
}

fn get_text<'a>(allocator: Allocator<'a>, node: &'a JSXAttributeName<'a>) -> &'a Atom<'a> {
  match node {
    JSXAttributeName::Identifier(node) => &node.name,
    JSXAttributeName::NamespacedName(node) => {
      let s = allocator.alloc_str(&format!("{}:{}", node.namespace.name, node.name.name));
      allocator.alloc(Atom::from(&*s))
    }
  }
}
