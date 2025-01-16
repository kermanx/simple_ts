use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::ForInStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_for_in_statement(&mut self, node: &'a ForInStatement<'a>) {
    self.exec_expression(&node.right);

    self.push_loop_scope();

    self.declare_for_statement_left(&node.left);

    self.init_for_statement_left(&node.left, Ty::String);
    self.exec_statement(&node.body);

    self.pop_scope();
  }
}
