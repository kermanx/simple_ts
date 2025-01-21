use oxc::ast::ast::LabeledStatement;

use crate::{analyzer::Analyzer, scope::cf::CfScopeKind};

impl<'a> Analyzer<'a> {
  pub fn declare_labeled_statement(&mut self, node: &'a LabeledStatement<'a>) {
    self.declare_statement(&node.body);
  }

  pub fn exec_labeled_statement(&mut self, node: &'a LabeledStatement<'a>) {
    self.push_scope(CfScopeKind::Labeled(node));
    self.exec_statement(&node.body);
    self.pop_scope();
  }
}
