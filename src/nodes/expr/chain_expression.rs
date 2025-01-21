use oxc::ast::{
  ast::{ChainElement, ChainExpression, Expression},
  match_member_expression,
};

use crate::{
  analyzer::Analyzer,
  ty::{union::into_union, Ty},
};

impl<'a> Analyzer<'a> {
  pub fn exec_chain_expression(
    &mut self,
    node: &'a ChainExpression<'a>,
    ty: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let (indeterminate, value) = match &node.expression {
      ChainElement::CallExpression(node) => self.exec_call_expression_in_chain(node, ty),
      ChainElement::TSNonNullExpression(node) => {
        let (indeterminate, value) = self.exec_expression_in_chain(&node.expression, ty);
        (indeterminate, self.non_nullable(value))
      }
      node => self.exec_member_expression_read_in_chain(node.to_member_expression(), None).0,
    };
    if indeterminate {
      self.pop_scope();
      into_union(self.allocator, [Ty::Undefined, value])
    } else {
      value
    }
  }

  pub fn exec_expression_in_chain(
    &mut self,
    node: &'a Expression<'a>,
    sat: Option<Ty<'a>>,
  ) -> (bool, Ty<'a>) {
    match node {
      match_member_expression!(Expression) => {
        self.exec_member_expression_read_in_chain(node.to_member_expression(), None).0
      }
      Expression::CallExpression(node) => self.exec_call_expression_in_chain(node, sat),
      Expression::TSNonNullExpression(node) => {
        let (indeterminate, value) = self.exec_expression_in_chain(&node.expression, sat);
        (indeterminate, self.non_nullable(value))
      }
      Expression::ChainExpression(_) => unreachable!(),
      _ => (false, self.exec_expression(node, sat)),
    }
  }
}
