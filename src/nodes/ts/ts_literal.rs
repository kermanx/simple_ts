use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TSLiteral;

impl<'a> Analyzer<'a> {
  pub fn resolve_literal(&mut self, node: &'a TSLiteral<'a>) -> Type<'a> {
    match node {
      TSLiteral::BooleanLiteral(node) => Type::BooleanLiteral(node.value),
      TSLiteral::NumericLiteral(node) => Type::NumericLiteral(node.value.into()),
      TSLiteral::StringLiteral(node) => Type::StringLiteral(&node.value),
      TSLiteral::BigIntLiteral(node) => Type::BigIntLiteral(&node.raw),
      TSLiteral::NullLiteral(_) => Type::Null,
      TSLiteral::RegExpLiteral(node) => todo!(),
      TSLiteral::UnaryExpression(node) => todo!(),
      TSLiteral::TemplateLiteral(node) => todo!(),
    }
  }
}
