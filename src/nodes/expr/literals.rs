use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::{
  BigIntLiteral, BooleanLiteral, NullLiteral, NumericLiteral, RegExpLiteral, StringLiteral,
};

impl<'a> Analyzer<'a> {
  pub fn exec_string_literal(&mut self, node: &'a StringLiteral) -> Type<'a> {
    Type::StringLiteral(&node.value)
  }

  pub fn exec_numeric_literal(&mut self, node: &'a NumericLiteral) -> Type<'a> {
    Type::NumberLiteral(node.value.into())
  }

  pub fn exc_big_int_literal(&mut self, node: &'a BigIntLiteral) -> Type<'a> {
    Type::BigIntLiteral(&node.raw)
  }

  pub fn exec_boolean_literal(&mut self, node: &'a BooleanLiteral) -> Type<'a> {
    Type::BooleanLiteral(node.value)
  }

  pub fn exec_null_literal(&mut self, _node: &'a NullLiteral) -> Type<'a> {
    Type::Null
  }

  pub fn exec_regexp_literal(&mut self, _node: &'a RegExpLiteral<'a>) -> Type<'a> {
    todo!()
  }
}
