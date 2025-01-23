use oxc::{
  ast::ast::{TSType, TSTypeOperatorOperator},
  span::SPAN,
};
use oxc_syntax::number::{BigintBase, NumberBase};

use super::Ty;
use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn print_type(&self, ty: Ty<'a>) -> TSType<'a> {
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
      Ty::UniqueSymbol(_) => self.ast_builder.ts_type_type_operator(
        SPAN,
        TSTypeOperatorOperator::Unique,
        self.ast_builder.ts_type_symbol_keyword(SPAN),
      ),

      Ty::Record(r) => self.print_record_type(r),
      Ty::Interface(i) => self.print_interface_type(i),
      Ty::Tuple(t) => self.print_tuple_type(t),
      Ty::Function(r) => self.print_callable_type(r),
      Ty::Constructor(r) => self.print_callable_type(r),
      Ty::Namespace(r) => self.print_namespace_type(r),

      Ty::Union(u) => self.print_union_type(u),
      Ty::Intersection(i) => self.print_intersection_type(i),

      Ty::Instance(i) => self.print_instance_type(i),
      Ty::Generic(g) => self.print_generic_type(g),
      Ty::Intrinsic(i) => self.print_intrinsic_type(i),

      Ty::Unresolved(u) => self.print_unresolved_type(u),
    }
  }
}
