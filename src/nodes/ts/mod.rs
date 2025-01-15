mod ts_literal;
mod ts_type_annotation;
mod ts_type_parameter_instantiation;
mod ts_type_query;
mod ts_type_reference;
mod ts_union_type;

use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TSType;

impl<'a> Analyzer<'a> {
  pub fn resolve_type(&mut self, node: &'a TSType<'a>) -> Option<Ty<'a>> {
    match node {
      TSType::TSAnyKeyword(_) => Some(Ty::Any),
      TSType::TSBigIntKeyword(_) => Some(Ty::BigInt),
      TSType::TSBooleanKeyword(_) => Some(Ty::Boolean),
      TSType::TSIntrinsicKeyword(_) => todo!(),
      TSType::TSNeverKeyword(_) => Some(Ty::Never),
      TSType::TSNullKeyword(_) => Some(Ty::Null),
      TSType::TSNumberKeyword(_) => Some(Ty::Number),
      TSType::TSObjectKeyword(_) => Some(Ty::Object),
      TSType::TSStringKeyword(_) => Some(Ty::String),
      TSType::TSSymbolKeyword(_) => Some(Ty::Symbol),
      TSType::TSUndefinedKeyword(_) => Some(Ty::Undefined),
      TSType::TSUnknownKeyword(_) => Some(Ty::Unknown),
      TSType::TSVoidKeyword(_) => Some(Ty::Void),

      TSType::TSLiteralType(node) => Some(self.resolve_literal(&node.literal)),
      TSType::TSTypeReference(node) => self.resolve_type_reference(node),
      TSType::TSTypeQuery(node) => self.resolve_type_query(node),
      TSType::TSUnionType(node) => self.resolve_union_type(node),
      TSType::TSParenthesizedType(node) => self.resolve_type(&node.type_annotation),

      _ => todo!(),
    }
  }
}
