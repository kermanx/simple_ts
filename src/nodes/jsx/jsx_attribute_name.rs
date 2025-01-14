use crate::{analyzer::Analyzer, r#type::Type};
use oxc::{allocator::Allocator, ast::ast::JSXAttributeName, span::Atom};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attribute_name(&mut self, node: &'a JSXAttributeName<'a>) -> Type<'a> {
    Type::StringLiteral(self.allocator.alloc(Atom::from(get_text(self.allocator, node))))
  }
}

fn get_text<'a>(allocator: &'a Allocator, node: &'a JSXAttributeName<'a>) -> &'a str {
  match node {
    JSXAttributeName::Identifier(node) => node.name.as_str(),
    JSXAttributeName::NamespacedName(node) => {
      allocator.alloc(format!("{}:{}", node.namespace.name, node.property.name))
    }
  }
}
