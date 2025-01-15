use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::{
  BigIntLiteral, BooleanLiteral, NullLiteral, NumericLiteral, RegExpLiteral, StringLiteral,
};

impl<'a> Analyzer<'a> {
  pub fn exec_string_literal(&mut self, node: &'a StringLiteral) -> Ty<'a> {
    Ty::StringLiteral(&node.value)
  }

  pub fn exec_numeric_literal(&mut self, node: &'a NumericLiteral) -> Ty<'a> {
    Ty::NumericLiteral(node.value.into())
  }

  pub fn exc_big_int_literal(&mut self, node: &'a BigIntLiteral) -> Ty<'a> {
    Ty::BigIntLiteral(&node.raw)
  }

  pub fn exec_boolean_literal(&mut self, node: &'a BooleanLiteral) -> Ty<'a> {
    Ty::BooleanLiteral(node.value)
  }

  pub fn exec_null_literal(&mut self, _node: &'a NullLiteral) -> Ty<'a> {
    Ty::Null
  }

  pub fn exec_regexp_literal(&mut self, _node: &'a RegExpLiteral<'a>) -> Ty<'a> {
    todo!()
  }
}
