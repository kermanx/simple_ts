use oxc::{
  ast::ast::{TSType, TSTypeOperatorOperator},
  span::SPAN,
};
use oxc_syntax::number::{BigintBase, NumberBase};

use super::Ty;
use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn serialize_type(&mut self, ty: Ty<'a>) -> TSType<'a> {
    match ty {
      Ty::Error | Ty::Any => self.ast_builder.ts_type_any_keyword(SPAN),
      Ty::Unknown => self.ast_builder.ts_type_unknown_keyword(SPAN),
      Ty::Never => self.ast_builder.ts_type_never_keyword(SPAN),
      Ty::Void => self.ast_builder.ts_type_void_keyword(SPAN),

      Ty::BigInt => self.ast_builder.ts_type_big_int_keyword(SPAN),
      Ty::Boolean => self.ast_builder.ts_type_boolean_keyword(SPAN),
      Ty::Null => self.ast_builder.ts_type_null_keyword(SPAN),
      Ty::Number => self.ast_builder.ts_type_number_keyword(SPAN),
      Ty::Object => self.ast_builder.ts_type_object_keyword(SPAN),
      Ty::String => self.ast_builder.ts_type_string_keyword(SPAN),
      Ty::Symbol => self.ast_builder.ts_type_symbol_keyword(SPAN),
      Ty::Undefined => self.ast_builder.ts_type_undefined_keyword(SPAN),

      Ty::StringLiteral(s) => self
        .ast_builder
        .ts_type_literal_type(SPAN, self.ast_builder.ts_literal_string_literal(SPAN, s, None)),
      Ty::NumericLiteral(n) => self.ast_builder.ts_type_literal_type(
        SPAN,
        self.ast_builder.ts_literal_numeric_literal(SPAN, n.0, None, NumberBase::Decimal),
      ),
      Ty::BigIntLiteral(n) => self.ast_builder.ts_type_literal_type(
        SPAN,
        self.ast_builder.ts_literal_big_int_literal(SPAN, n, BigintBase::Decimal),
      ),
      Ty::BooleanLiteral(b) => self
        .ast_builder
        .ts_type_literal_type(SPAN, self.ast_builder.ts_literal_boolean_literal(SPAN, b)),
      Ty::UniqueSymbol(_) => self.ast_builder.ts_type_type_operator_type(
        SPAN,
        TSTypeOperatorOperator::Unique,
        self.ast_builder.ts_type_symbol_keyword(SPAN),
      ),

      Ty::Record(r) => self.serialize_record_type(r),
      Ty::Interface(i) => self.serialize_interface_type(i),
      Ty::Tuple(t) => self.serialize_tuple_type(t),
      Ty::Function(r) => self.serialize_callable_type(r),
      Ty::Constructor(r) => self.serialize_callable_type(r),
      Ty::Namespace(r) => self.serialize_namespace_type(r),

      Ty::Union(u) => self.serialize_union_type(u),
      Ty::Intersection(i) => self.serialize_intersection_type(i),

      Ty::Instance(i) => self.serialize_instance_type(i),
      Ty::Generic(g) => self.serialize_generic_type(g),
      Ty::Intrinsic(i) => self.serialize_intrinsic_type(i),

      Ty::Unresolved(u) => self.serialize_unresolved_type(u),
    }
  }
}
