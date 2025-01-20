mod ts_as_expression;
mod ts_infer_type;
mod ts_instantiation_expression;
mod ts_literal;
mod ts_type_annotation;
mod ts_type_literal;
mod ts_type_parameter_declaration;
mod ts_type_parameter_instantiation;
mod ts_type_query;
mod ts_type_reference;
mod ts_union_type;

use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TSType;

impl<'a> Analyzer<'a> {
  pub fn resolve_type(&mut self, node: &'a TSType<'a>) -> Ty<'a> {
    match node {
      TSType::TSAnyKeyword(_) => Ty::Any,
      TSType::TSBigIntKeyword(_) => Ty::BigInt,
      TSType::TSBooleanKeyword(_) => Ty::Boolean,
      TSType::TSIntrinsicKeyword(_) => todo!(),
      TSType::TSNeverKeyword(_) => Ty::Never,
      TSType::TSNullKeyword(_) => Ty::Null,
      TSType::TSNumberKeyword(_) => Ty::Number,
      TSType::TSObjectKeyword(_) => Ty::Object,
      TSType::TSStringKeyword(_) => Ty::String,
      TSType::TSSymbolKeyword(_) => Ty::Symbol,
      TSType::TSUndefinedKeyword(_) => Ty::Undefined,
      TSType::TSUnknownKeyword(_) => Ty::Unknown,
      TSType::TSVoidKeyword(_) => Ty::Void,

      TSType::TSLiteralType(node) => self.resolve_literal(&node.literal),
      TSType::TSTypeReference(node) => self.resolve_type_reference(node),
      TSType::TSTypeQuery(node) => self.resolve_type_query(node),
      TSType::TSUnionType(node) => self.resolve_union_type(node),
      TSType::TSParenthesizedType(node) => self.resolve_type(&node.type_annotation),
      TSType::TSTypeLiteral(node) => self.resolve_type_literal(node),
      TSType::TSInferType(node) => self.resolve_infer_type(node),

      _ => todo!(),
    }
  }
}
