use crate::analyzer::Analyzer;
use oxc::ast::ast::BlockStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_block_statement(&mut self, node: &'a BlockStatement) {
    self.exec_statement_vec(&node.body);
  }
}
