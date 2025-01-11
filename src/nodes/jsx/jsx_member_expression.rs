use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::JSXMemberExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_member_expression(&mut self, node: &'a JSXMemberExpression<'a>) -> Entity<'a> {
    let object = self.exec_jsx_member_expression_object(&node.object);
    let key = self.factory.string_literal(&node.property.name);
    object.get_property(self, key)
  }
}
