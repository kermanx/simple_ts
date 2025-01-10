use crate::analyzer::Analyzer;
use oxc::{allocator::Vec, ast::ast::Statement};

impl<'a> Analyzer<'a> {
  pub fn exec_statement_vec(&mut self, statements: &'a Vec<'a, Statement<'a>>) {
    for statement in statements {
      self.declare_statement(statement);
    }

    for statement in statements {
      if self.cf_scope().must_exited() {
        break;
      }
      self.init_statement(statement);
    }
  }
}
