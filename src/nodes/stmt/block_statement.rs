use oxc::ast::ast::BlockStatement;

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_block_statement(&mut self, node: &'a BlockStatement) {
    self.exec_statement_vec(&node.body);
  }
}
