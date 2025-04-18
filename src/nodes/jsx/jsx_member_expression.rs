use oxc::ast::ast::JSXMemberExpression;

use crate::{
  analyzer::Analyzer,
  ty::{Ty, property_key::PropertyKeyType},
};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_member_expression(
    &mut self,
    node: &'a JSXMemberExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let object = self.exec_jsx_member_expression_object(&node.object, None);
    let key = PropertyKeyType::StringLiteral(&node.property.name);
    self.get_property(object, key)
  }
}
