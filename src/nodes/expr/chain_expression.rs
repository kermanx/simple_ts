use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::{
  ast::{ChainElement, ChainExpression, Expression},
  match_member_expression,
};

impl<'a> Analyzer<'a> {
  pub fn exec_chain_expression(&mut self, node: &'a ChainExpression<'a>) -> Entity<'a> {
    match &node.expression {
      ChainElement::CallExpression(node) => {
        let result = self.exec_call_expression_in_chain(node);
        match result {
          Ok((scope_count, value, undefined)) => {
            self.pop_multiple_cf_scopes(scope_count);
            self.factory.optional_union(value, undefined)
          }
          Err(value) => value,
        }
      }
      node => {
        let result = self.exec_member_expression_read_in_chain(node.to_member_expression());
        match result {
          Ok((scope_count, value, undefined, _)) => {
            self.pop_multiple_cf_scopes(scope_count);
            self.factory.optional_union(value, undefined)
          }
          Err(value) => value,
        }
      }
    }
  }

  pub fn exec_expression_in_chain(
    &mut self,
    node: &'a Expression<'a>,
  ) -> Result<(usize, Entity<'a>, Option<Entity<'a>>), Entity<'a>> {
    match node {
      match_member_expression!(Expression) => self
        .exec_member_expression_read_in_chain(node.to_member_expression())
        .map(|(scope_count, value, undefined, _)| (scope_count, value, undefined)),
      Expression::CallExpression(node) => self.exec_call_expression_in_chain(node),
      _ => Ok((0, self.exec_expression(node), None)),
    }
  }
}
