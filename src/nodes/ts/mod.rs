mod ts_type_annotation;
mod ts_literal;

use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::TSType;

impl<'a> Analyzer<'a> {
  pub fn exec_ts_type(&mut self, node: &'a TSType<'a>) -> Entity<'a> {
    match node {
      TSType::TSAnyKeyword(_) => self.factory.any,
      TSType::TSBigIntKeyword(_) => self.factory.bigint,
      TSType::TSBooleanKeyword(_) => self.factory.boolean,
      TSType::TSIntrinsicKeyword(_) => todo!(),
      TSType::TSNeverKeyword(_) => self.factory.never,
      TSType::TSNullKeyword(_) => self.factory.null,
      TSType::TSNumberKeyword(_) => self.factory.number,
      TSType::TSObjectKeyword(_) => self.factory.object,
      TSType::TSStringKeyword(_) => self.factory.string,
      TSType::TSSymbolKeyword(_) => self.factory.symbol,
      TSType::TSUndefinedKeyword(_) => self.factory.undefined,
      TSType::TSUnknownKeyword(_) => self.factory.unknown,
      TSType::TSVoidKeyword(_) => self.factory.void,

      TSType::TSLiteralType(node) => self.exec_ts_literal(&node.literal),

      _ => todo!(),
    }
  }
}
