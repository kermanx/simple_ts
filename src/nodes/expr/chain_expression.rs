use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};
use oxc::ast::{
  ast::{ChainElement, ChainExpression, Expression},
  match_member_expression,
};

impl<'a> Analyzer<'a> {
  pub fn exec_chain_expression(&mut self, node: &'a ChainExpression<'a>) -> Ty<'a> {
    let (indeterminate, value) = match &node.expression {
      ChainElement::CallExpression(node) => self.exec_call_expression_in_chain(node),
      node => self.exec_member_expression_read_in_chain(node.to_member_expression()).0,
    };
    if indeterminate {
      self.pop_cf_scope();
      into_union(self.allocator, [Ty::Undefined, value])
    } else {
      value
    }
  }

  pub fn exec_expression_in_chain(&mut self, node: &'a Expression<'a>) -> (bool, Ty<'a>) {
    match node {
      match_member_expression!(Expression) => {
        self.exec_member_expression_read_in_chain(node.to_member_expression()).0
      }
      Expression::CallExpression(node) => self.exec_call_expression_in_chain(node),
      _ => (false, self.exec_expression(node)),
    }
  }
}
