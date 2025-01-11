use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::{
  BigIntLiteral, BooleanLiteral, NullLiteral, NumberBase, NumericLiteral, RegExpLiteral,
  StringLiteral,
};

impl<'a> Analyzer<'a> {
  pub fn exec_string_literal(&mut self, node: &'a StringLiteral) -> Entity<'a> {
    self.factory.string_literal(&node.value)
  }

  pub fn exec_numeric_literal(&mut self, node: &'a NumericLiteral) -> Entity<'a> {
    if node.base == NumberBase::Float {
      self.factory.number
    } else {
      self.factory.numeric_literal(node.value)
    }
  }

  pub fn exc_big_int_literal(&mut self, node: &'a BigIntLiteral) -> Entity<'a> {
    self.factory.big_int_literal(&node.raw)
  }

  pub fn exec_boolean_literal(&mut self, node: &'a BooleanLiteral) -> Entity<'a> {
    self.factory.boolean_literal(node.value)
  }

  pub fn exec_null_literal(&mut self, _node: &'a NullLiteral) -> Entity<'a> {
    self.factory.null
  }

  pub fn exec_regexp_literal(&mut self, _node: &'a RegExpLiteral<'a>) -> Entity<'a> {
    self.factory.unknown
  }
}
