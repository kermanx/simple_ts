use crate::{
  analyzer::Analyzer,
  ty::{property_key::PropertyKeyType, Ty},
};
use oxc::ast::ast::MemberExpression;

impl<'a> Analyzer<'a> {
  /// Returns (short-circuit, value, cache)
  pub fn exec_member_expression_read(
    &mut self,
    node: &'a MemberExpression<'a>,
    sat: Option<Ty<'a>>,
  ) -> (Ty<'a>, (Ty<'a>, PropertyKeyType<'a>)) {
    let ((indeterminate, value), cache) = self.exec_member_expression_read_in_chain(node, sat);

    if indeterminate {
      self.pop_scope();
    }

    (value, cache)
  }

  /// Returns ((indeterminate, value), cache)
  pub fn exec_member_expression_read_in_chain(
    &mut self,
    node: &'a MemberExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> ((bool, Ty<'a>), (Ty<'a>, PropertyKeyType<'a>)) {
    let (mut indeterminate, object) = self.exec_expression_in_chain(node.object(), None);

    if !indeterminate && node.optional() {
      self.push_indeterminate_scope();
      indeterminate = true;
    }

    let key = self.exec_key(node);

    let value = self.get_property(object, key);

    ((indeterminate, value), (object, key))
  }

  pub fn exec_member_expression_write(
    &mut self,
    node: &'a MemberExpression<'a>,
    value: Ty<'a>,
    cache: Option<(Ty<'a>, PropertyKeyType<'a>)>,
  ) {
    let (object, key) = cache.unwrap_or_else(|| {
      let object = self.exec_expression(node.object(), None);
      let key = self.exec_key(node);
      (object, key)
    });

    self.set_property(object, key, value);
  }

  fn exec_key(&mut self, node: &'a MemberExpression<'a>) -> PropertyKeyType<'a> {
    let value = match node {
      MemberExpression::ComputedMemberExpression(node) => {
        self.exec_expression(&node.expression, None)
      }
      MemberExpression::StaticMemberExpression(node) => self.exec_identifier_name(&node.property),
      MemberExpression::PrivateFieldExpression(node) => self.exec_private_identifier(&node.field),
    };
    self.to_property_key(value)
  }
}
