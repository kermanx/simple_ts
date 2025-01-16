use crate::analyzer::Analyzer;
use oxc::ast::ast::WhileStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_while_statement(&mut self, node: &'a WhileStatement<'a>) {
    self.push_indeterminate_scope();
    self.exec_expression(&node.test);
    // CHECKER
    self.pop_scope();

    self.push_loop_scope();
    self.exec_statement(&node.body);
    self.pop_scope();
  }
}
