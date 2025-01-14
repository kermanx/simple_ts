use super::Type;
use crate::analyzer::Analyzer;
use oxc_syntax::operator::BinaryOperator;

impl<'a> Analyzer<'a> {
  pub fn binary_operation(
    &mut self,
    operator: BinaryOperator,
    lhs: Type<'a>,
    rhs: Type<'a>,
  ) -> Type<'a> {
    todo!()
  }
}
