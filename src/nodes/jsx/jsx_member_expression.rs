use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::JSXMemberExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_member_expression(&mut self, node: &'a JSXMemberExpression<'a>) -> Type<'a> {
    let object = self.exec_jsx_member_expression_object(&node.object);
    let key = Type::StringLiteral(&node.property.name);
    self.get_property(object, key)
  }
}
