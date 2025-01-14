use crate::analyzer::Analyzer;
use oxc::ast::ast::ForOfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_for_of_statement(&mut self, node: &'a ForOfStatement<'a>) {
    let right = self.exec_expression(&node.right);
    let iterated = self.iterate_result_union(right);

    // FIXME: node.r#await

    self.push_variable_scope();

    self.declare_for_statement_left(&node.left);

    self.push_loop_cf_scope();

    self.init_for_statement_left(&node.left, iterated);
    self.exec_statement(&node.body);

    self.pop_cf_scope();

    self.pop_variable_scope();
  }
}
