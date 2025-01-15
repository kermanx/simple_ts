use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::JSXMemberExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_member_expression(&mut self, node: &'a JSXMemberExpression<'a>) -> Ty<'a> {
    let object = self.exec_jsx_member_expression_object(&node.object);
    let key = Ty::StringLiteral(&node.property.name);
    self.get_property(object, key)
  }
}
