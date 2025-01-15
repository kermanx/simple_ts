use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::{
  ast::{ChainElement, Expression, MemberExpression},
  match_member_expression,
};

/// Returns Some((node, same_chain))
fn unwrap_to_member_expression<'a>(
  node: &'a Expression<'a>,
) -> Option<(&'a MemberExpression<'a>, bool)> {
  match node {
    match_member_expression!(Expression) => Some((node.to_member_expression(), true)),
    Expression::ParenthesizedExpression(node) => {
      unwrap_to_member_expression(&node.expression).map(|(node, _)| (node, false))
    }
    Expression::ChainExpression(node) => match &node.expression {
      match_member_expression!(ChainElement) => {
        Some((node.expression.to_member_expression(), false))
      }
      _ => None,
    },
    _ => None,
  }
}

impl<'a> Analyzer<'a> {
  pub fn exec_callee(&mut self, node: &'a Expression<'a>) -> (bool, Ty<'a>, Ty<'a>) {
    if let Some((member_expr, same_chain)) = unwrap_to_member_expression(node) {
      if same_chain {
        let ((indeterminate, callee), (object, _)) =
          self.exec_member_expression_read_in_chain(member_expr);
        (indeterminate, callee, object)
      } else {
        let (callee, (object, _)) = self.exec_member_expression_read(member_expr);
        (false, callee, object)
      }
    } else {
      let (indeterminate, callee) = self.exec_expression_in_chain(node);
      (indeterminate, callee, Ty::Undefined)
    }
  }
}
