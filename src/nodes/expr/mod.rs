mod array_expression;
mod arrow_function_expression;
mod assignment_expression;
mod await_expression;
mod binary_expression;
mod call_expression;
mod chain_expression;
mod conditional_expression;
mod import_expression;
mod literals;
mod logical_expression;
mod member_expression;
mod meta_property;
mod new_expression;
mod object_expression;
mod parenthesized_expression;
mod private_in_expression;
mod sequence_expression;
mod super_expression;
mod tagged_template_expression;
mod template_literal;
mod this_expression;
mod unary_expression;
mod update_expression;
mod yield_expression;

use oxc::{
  ast::{ast::Expression, match_member_expression},
  span::GetSpan,
};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_expression(&mut self, node: &'a Expression<'a>, sat: Option<Ty<'a>>) -> Ty<'a> {
    let span = node.span();
    self.push_span(&span);
    let value = self.exec_expression_impl(node, sat);
    self.accumulate_type(&span, value);
    self.pop_span();
    value
  }

  pub fn exec_expression_with_as_const(
    &mut self,
    node: &'a Expression<'a>,
    sat: Option<Ty<'a>>,
    as_const: bool,
  ) -> Ty<'a> {
    if !as_const {
      return self.exec_expression(node, sat);
    }

    let span = node.span();
    self.push_span(&span);

    let value = match node {
      match_member_expression!(Expression) => {
        // TODO: Enum member
        todo!()
      }
      Expression::StringLiteral(node) => Ty::StringLiteral(&node.value),
      Expression::NumericLiteral(node) => Ty::NumericLiteral(node.value.into()),
      Expression::BigIntLiteral(node) => Ty::BigIntLiteral(&node.raw),
      Expression::BooleanLiteral(node) => Ty::BooleanLiteral(node.value),

      Expression::ObjectExpression(node) => self.exec_object_expression(node, sat, true),
      Expression::ArrayExpression(node) => self.exec_array_expression(node, sat, true),
      Expression::ParenthesizedExpression(node) => {
        self.exec_parenthesized_expression(node, sat, true)
      }

      _ => self.exec_expression_impl(node, sat),
    };

    self.accumulate_type(&span, value);

    self.pop_span();

    value
  }

  fn exec_expression_impl(&mut self, node: &'a Expression<'a>, sat: Option<Ty<'a>>) -> Ty<'a> {
    match node {
      match_member_expression!(Expression) => {
        self.exec_member_expression_read(node.to_member_expression(), sat).0
      }
      Expression::StringLiteral(_) => Ty::String,
      Expression::NumericLiteral(_) => Ty::Number,
      Expression::BigIntLiteral(_) => Ty::BigInt,
      Expression::BooleanLiteral(_) => Ty::Boolean,
      Expression::NullLiteral(_) => Ty::Null,
      Expression::RegExpLiteral(_) => todo!("RegExp type"),
      Expression::TemplateLiteral(node) => self.exec_template_literal(node, sat),
      Expression::Identifier(node) => self.exec_identifier_reference_read(node, sat),
      Expression::FunctionExpression(node) => self.exec_function(node, sat),
      Expression::ArrowFunctionExpression(node) => self.exec_arrow_function_expression(node, sat),
      Expression::UnaryExpression(node) => self.exec_unary_expression(node, sat),
      Expression::UpdateExpression(node) => self.exec_update_expression(node, sat),
      Expression::BinaryExpression(node) => self.exec_binary_expression(node, sat),
      Expression::LogicalExpression(node) => self.exec_logical_expression(node, sat),
      Expression::ConditionalExpression(node) => self.exec_conditional_expression(node, sat),
      Expression::CallExpression(node) => self.exec_call_expression(node, sat),
      Expression::TaggedTemplateExpression(node) => self.exec_tagged_template_expression(node, sat),
      Expression::AwaitExpression(node) => self.exec_await_expression(node, sat),
      Expression::YieldExpression(node) => self.exec_yield_expression(node, sat),
      Expression::ObjectExpression(node) => self.exec_object_expression(node, sat, false),
      Expression::ArrayExpression(node) => self.exec_array_expression(node, sat, false),
      Expression::ParenthesizedExpression(node) => {
        self.exec_parenthesized_expression(node, sat, false)
      }
      Expression::SequenceExpression(node) => self.exec_sequence_expression(node, sat),
      Expression::AssignmentExpression(node) => self.exec_assignment_expression(node, sat),
      Expression::ChainExpression(node) => self.exec_chain_expression(node, sat),
      Expression::ImportExpression(node) => self.exec_import_expression(node, sat),
      Expression::MetaProperty(node) => self.exec_meta_property(node, sat),
      Expression::NewExpression(node) => self.exec_new_expression(node, sat),
      Expression::ClassExpression(node) => self.exec_class(node, sat),
      Expression::ThisExpression(node) => self.exec_this_expression(node, sat),
      Expression::Super(node) => self.exec_super(node, sat),
      Expression::PrivateInExpression(node) => self.exec_private_in_expression(node, sat),

      Expression::JSXElement(node) => self.exec_jsx_element(node, sat),
      Expression::JSXFragment(node) => self.exec_jsx_fragment(node, sat),

      Expression::TSAsExpression(node) => self.exec_ts_as_expression(node, sat),
      Expression::TSSatisfiesExpression(node) => self.exec_ts_satisfies_expression(node, sat),
      Expression::TSTypeAssertion(node) => self.exec_ts_type_assertion(node, sat),
      Expression::TSNonNullExpression(node) => self.exec_ts_non_null_expression(node, sat),
      Expression::TSInstantiationExpression(node) => {
        self.exec_ts_instantiation_expression(node, sat)
      }

      Expression::V8IntrinsicExpression(_) => unreachable!(),
    }
  }
}
