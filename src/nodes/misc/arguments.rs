use crate::{analyzer::Analyzer, r#type::Type};
use oxc::{allocator, ast::ast::Argument};

impl<'a> Analyzer<'a> {
  pub fn exec_arguments(&mut self, node: &'a allocator::Vec<'a, Argument<'a>>) -> Type<'a> {
    todo!()
  }
}
