use oxc::ast::ast::JSXElement;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_element(&mut self, node: &'a JSXElement<'a>, _sat: Option<Ty<'a>>) -> Ty<'a> {
    let tag = self.exec_jsx_element_name(&node.opening_element.name);
    let attr_sat = self
      .extract_callable_function(tag)
      .map(|func| self.get_callable_parameter_types(self.type_scopes.constraints_scope, &func)[0]);
    let attributes = self.exec_jsx_attributes(&node.opening_element, attr_sat);
    let children = self.exec_jsx_children(&node.children);
    // attributes.init_property(
    //   self,
    //   PropertyKind::Init,
    //   Ty::StringLiteral("children"),
    //   children,
    //   true,
    // );
    // self.factory.react_element(tag, attributes)
    todo!()
  }
}
