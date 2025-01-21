use oxc::ast::ast::StaticBlock;

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_static_block(&mut self, node: &'a StaticBlock<'a>) {
    self.exec_statement_vec(&node.body);
  }
}
