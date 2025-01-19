use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::JSXMemberExpressionObject;

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_member_expression_object(
    &mut self,
    node: &'a JSXMemberExpressionObject<'a>,
    ty: Option<Ty<'a>>,
  ) -> Ty<'a> {
    match node {
      JSXMemberExpressionObject::IdentifierReference(node) => {
        self.exec_identifier_reference_read(node, ty)
      }
      JSXMemberExpressionObject::MemberExpression(node) => {
        self.exec_jsx_member_expression(node, ty)
      }
      JSXMemberExpressionObject::ThisExpression(node) => self.exec_this_expression(node, ty),
    }
  }
}
