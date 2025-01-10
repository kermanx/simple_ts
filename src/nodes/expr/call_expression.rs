use crate::{analyzer::Analyzer, entity::Entity, scope::CfScopeKind};
use oxc::ast::ast::CallExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_call_expression(&mut self, node: &'a CallExpression) -> Entity<'a> {
    let (scope_count, value, undefined) = self.exec_call_expression_in_chain(node).unwrap();

    assert_eq!(scope_count, 0);
    assert!(undefined.is_none());

    value
  }

  /// Returns (short-circuit, value)
  pub fn exec_call_expression_in_chain(
    &mut self,
    node: &'a CallExpression,
  ) -> Result<(usize, Entity<'a>, Option<Entity<'a>>), Entity<'a>> {
    let (mut scope_count, callee, mut undefined, this) = self.exec_callee(&node.callee)?;

    if node.optional {
      let maybe_left = match callee.test_nullish() {
        Some(true) => {
          self.pop_multiple_cf_scopes(scope_count);
          return Err(self.factory.undefined);
        }
        Some(false) => false,
        None => {
          undefined = Some(self.factory.undefined);
          true
        }
      };

      self.push_cf_scope(
        CfScopeKind::LogicalRight,
        None,
        if maybe_left { None } else { Some(false) },
      );

      scope_count += 1;
    }

    let args = self.exec_arguments(&node.arguments);

    let ret_val = callee.call(self, this, args);

    Ok((scope_count, ret_val, undefined))
  }
}
