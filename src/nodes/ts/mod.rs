mod ts_literal;
mod ts_type_annotation;
mod ts_type_parameter_instantiation;
mod ts_type_query;
mod ts_type_reference;

use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TSType;

impl<'a> Analyzer<'a> {
  pub fn resolve_type(&mut self, node: &'a TSType<'a>) -> Option<Type<'a>> {
    match node {
      TSType::TSAnyKeyword(_) => Some(Type::Any),
      TSType::TSBigIntKeyword(_) => Some(Type::BigInt),
      TSType::TSBooleanKeyword(_) => Some(Type::Boolean),
      TSType::TSIntrinsicKeyword(_) => todo!(),
      TSType::TSNeverKeyword(_) => Some(Type::Never),
      TSType::TSNullKeyword(_) => Some(Type::Null),
      TSType::TSNumberKeyword(_) => Some(Type::Number),
      TSType::TSObjectKeyword(_) => Some(Type::Object),
      TSType::TSStringKeyword(_) => Some(Type::String),
      TSType::TSSymbolKeyword(_) => Some(Type::Symbol),
      TSType::TSUndefinedKeyword(_) => Some(Type::Undefined),
      TSType::TSUnknownKeyword(_) => Some(Type::Unknown),
      TSType::TSVoidKeyword(_) => Some(Type::Void),

      TSType::TSLiteralType(node) => Some(self.resolve_literal(&node.literal)),
      TSType::TSTypeReference(node) => self.resolve_type_reference(node),

      _ => todo!(),
    }
  }
}
