use crate::{analyzer::Analyzer, r#type::Type, scope::CfScopeKind};
use oxc::ast::ast::MemberExpression;

impl<'a> Analyzer<'a> {
  /// Returns (short-circuit, value, cache)
  pub fn exec_member_expression_read(
    &mut self,
    node: &'a MemberExpression<'a>,
  ) -> (Type<'a>, (Type<'a>, Type<'a>)) {
    let (scope_count, value, undefined, cache) =
      self.exec_member_expression_read_in_chain(node).unwrap();

    assert_eq!(scope_count, 0);
    assert!(undefined.is_none());

    (value, cache)
  }

  /// Returns (scope_count, value, forwarded_undefined, cache)
  pub fn exec_member_expression_read_in_chain(
    &mut self,
    node: &'a MemberExpression<'a>,
  ) -> Result<(usize, Type<'a>, Option<Type<'a>>, (Type<'a>, Type<'a>)), Type<'a>> {
    let (mut scope_count, object, mut undefined) = self.exec_expression_in_chain(node.object())?;

    if node.optional() {
      let maybe_left = match object.test_nullish() {
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

    let key = self.exec_key(node);

    let value = object.get_property(self, key);

    Ok((scope_count, value, undefined, (object, key)))
  }

  pub fn exec_member_expression_write(
    &mut self,
    node: &'a MemberExpression<'a>,
    value: Type<'a>,
    cache: Option<(Type<'a>, Type<'a>)>,
  ) {
    let (object, key) = cache.unwrap_or_else(|| {
      let object = self.exec_expression(node.object());

      let key = self.exec_key(node);

      (object, key)
    });

    object.set_property(self, key, value);
  }

  fn exec_key(&mut self, node: &'a MemberExpression<'a>) -> Type<'a> {
    match node {
      MemberExpression::ComputedMemberExpression(node) => self.exec_expression(&node.expression),
      MemberExpression::StaticMemberExpression(node) => self.exec_identifier_name(&node.property),
      MemberExpression::PrivateFieldExpression(node) => self.exec_private_identifier(&node.field),
    }
  }
}
