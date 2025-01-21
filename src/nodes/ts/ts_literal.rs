use oxc::ast::ast::TSLiteral;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_literal(&mut self, node: &'a TSLiteral<'a>) -> Ty<'a> {
    match node {
      TSLiteral::BooleanLiteral(node) => Ty::BooleanLiteral(node.value),
      TSLiteral::NumericLiteral(node) => Ty::NumericLiteral(node.value.into()),
      TSLiteral::StringLiteral(node) => Ty::StringLiteral(&node.value),
      TSLiteral::BigIntLiteral(node) => Ty::BigIntLiteral(&node.raw),
      TSLiteral::NullLiteral(_) => Ty::Null,
      TSLiteral::RegExpLiteral(node) => todo!(),
      TSLiteral::UnaryExpression(node) => todo!(),
      TSLiteral::TemplateLiteral(node) => todo!(),
    }
  }
}
