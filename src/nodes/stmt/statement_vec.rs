use oxc::{allocator::Vec, ast::ast::Statement};

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_statement_vec(&mut self, statements: &'a Vec<'a, Statement<'a>>) {
    for statement in statements {
      self.declare_statement(statement);
    }

    for statement in statements {
      self.init_statement(statement);
    }
  }
}
