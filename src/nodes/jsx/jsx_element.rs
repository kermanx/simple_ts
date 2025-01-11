use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::{JSXElement, PropertyKind};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_element(&mut self, node: &'a JSXElement<'a>) -> Type<'a> {
    let tag = self.exec_jsx_element_name(&node.opening_element.name);
    let attributes = self.exec_jsx_attributes(&node.opening_element);
    let children = self.exec_jsx_children(&node.children);
    attributes.init_property(
      self,
      PropertyKind::Init,
      self.factory.string_literal("children"),
      children,
      true,
    );
    self.factory.react_element(tag, attributes)
  }
}
