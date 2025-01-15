use super::Ty;
use crate::analyzer::Analyzer;
use oxc_syntax::operator::BinaryOperator;

impl<'a> Analyzer<'a> {
  pub fn binary_operation(&mut self, operator: BinaryOperator, lhs: Ty<'a>, rhs: Ty<'a>) -> Ty<'a> {
    todo!()
  }
}
