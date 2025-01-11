use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TSLiteral;

impl<'a> Analyzer<'a> {
  pub fn exec_ts_literal(&mut self, node: &'a TSLiteral<'a>) -> Type<'a> {
    match node {
      TSLiteral::BooleanLiteral(node) => {
        if node.value {
          self.factory.true_literal
        } else {
          self.factory.false_literal
        }
      }
      TSLiteral::NumericLiteral(node) => self.factory.numeric_literal(node.value),
      TSLiteral::StringLiteral(node) => self.factory.string_literal(&node.value),
      TSLiteral::BigIntLiteral(node) => self.factory.big_int_literal(&node.raw),
      TSLiteral::NullLiteral(node) => self.factory.null,
      TSLiteral::RegExpLiteral(node) => todo!(),
      TSLiteral::UnaryExpression(node) => todo!(),
      TSLiteral::TemplateLiteral(node) => todo!(),
    }
  }
}
