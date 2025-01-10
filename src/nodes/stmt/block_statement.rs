use crate::{analyzer::Analyzer, scope::CfScopeKind};
use oxc::ast::ast::BlockStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_block_statement(&mut self, node: &'a BlockStatement) {
    let labels = self.take_labels();

    self.push_cf_scope(CfScopeKind::Block, labels, Some(false));
    self.exec_statement_vec(&node.body);
    self.pop_cf_scope();
  }
}
