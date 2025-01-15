use crate::{analyzer::Analyzer, ty::Ty};
use oxc::{allocator, ast::ast::Argument};

impl<'a> Analyzer<'a> {
  pub fn exec_arguments(&mut self, node: &'a allocator::Vec<'a, Argument<'a>>) -> Ty<'a> {
    todo!()
  }
}
