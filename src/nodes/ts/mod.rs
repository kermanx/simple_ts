mod ts_as_expression;
mod ts_conditional_type;
mod ts_constructor_type;
mod ts_function_type;
mod ts_indexed_access_type;
mod ts_infer_type;
mod ts_instantiation_expression;
mod ts_interface_declaration;
mod ts_intersection_type;
mod ts_literal;
mod ts_non_null_expression;
mod ts_operator_type;
mod ts_satisfies_expression;
mod ts_signature_vec;
mod ts_tuple_element;
mod ts_tuple_type;
mod ts_type_alias;
mod ts_type_annotation;
mod ts_type_assertion;
mod ts_type_literal;
mod ts_type_parameter_declaration;
mod ts_type_parameter_instantiation;
mod ts_type_query;
mod ts_type_reference;
mod ts_union_type;

use oxc::ast::ast::TSType;

use crate::{analyzer::Analyzer, ty::Ty};

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
      TSType::TSIntersectionType(node) => self.resolve_intersection_type(node),
      TSType::TSParenthesizedType(node) => self.resolve_type(&node.type_annotation),
      TSType::TSTypeLiteral(node) => self.resolve_type_literal(node),
      TSType::TSInferType(node) => self.resolve_infer_type(node),
      TSType::TSFunctionType(node) => self.resolve_function_type(node),
      TSType::TSConstructorType(node) => self.resolve_constructor_type(node),
      TSType::TSConditionalType(node) => self.resolve_conditional_type(node),
      TSType::TSTypeOperatorType(node) => self.resolve_operator_type(node),
      TSType::TSTupleType(node) => self.resolve_tuple_type(node, false),
      TSType::TSIndexedAccessType(node) => self.resolve_indexed_access_type(node),
      TSType::TSNamedTupleMember(_) => unreachable!("Handled in TSTupleElement"),

      _ => todo!("node: {:#?}", node),
    }
  }
}
