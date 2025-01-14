use crate::analyzer::Analyzer;
use oxc::ast::ast::DoWhileStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_do_while_statement(&mut self, node: &'a DoWhileStatement<'a>) {
    self.exec_statement(&node.body);

    self.push_indeterminate_cf_scope();
    self.exec_expression(&node.test);
    // CHECKER
    self.pop_cf_scope();

    self.push_loop_cf_scope();
    self.exec_statement(&node.body);
    self.pop_cf_scope();
  }
}
