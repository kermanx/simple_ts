use crate::{analyzer::Analyzer, entity::Entity};
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
  /// Returns: Ok((scope_count, callee, undefined, this)) or Err(forwarded_undefined) for should not call due to ?. operator
  pub fn exec_callee(
    &mut self,
    node: &'a Expression<'a>,
  ) -> Result<(usize, Entity<'a>, Option<Entity<'a>>, Entity<'a>), Entity<'a>> {
    if let Some((member_expr, same_chain)) = unwrap_to_member_expression(node) {
      if same_chain {
        let (scope_count, callee, undefined, (object, _)) =
          self.exec_member_expression_read_in_chain(member_expr)?;
        Ok((scope_count, callee, undefined, object))
      } else {
        let result = self.exec_member_expression_read_in_chain(member_expr);
        Ok(match result {
          Ok((scope_count, value, undefined, (object, _))) => {
            self.pop_multiple_cf_scopes(scope_count);
            (0, self.factory.optional_union(value, undefined), None, object)
          }
          Err(value) => (0, value, None, self.factory.unknown),
        })
      }
    } else {
      let (scope_count, callee, undefined) = self.exec_expression_in_chain(node)?;
      Ok((scope_count, callee, undefined, self.factory.undefined))
    }
  }
}
