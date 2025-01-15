use oxc::ast::ast::TSType;

use super::Ty;
use crate::analyzer::Analyzer;

#[derive(Debug)]
pub struct IntersectionType<'a> {
  pub types: Vec<Ty<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn print_intersection_type(&self, intersection: &IntersectionType<'a>) -> TSType<'a> {
    todo!()
  }
}
