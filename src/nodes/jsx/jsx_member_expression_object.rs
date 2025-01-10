use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::JSXMemberExpressionObject;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_member_expression_object(
    &mut self,
    node: &'a JSXMemberExpressionObject<'a>,
  ) -> Entity<'a> {
    match node {
      JSXMemberExpressionObject::IdentifierReference(node) => {
        self.exec_identifier_reference_read(node)
      }
      JSXMemberExpressionObject::MemberExpression(node) => self.exec_jsx_member_expression(node),
      JSXMemberExpressionObject::ThisExpression(node) => self.exec_this_expression(node),
    }
  }
}
