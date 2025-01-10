use crate::analyzer::Analyzer;
use oxc::ast::ast::LabeledStatement;

impl<'a> Analyzer<'a> {
  pub fn declare_labeled_statement(&mut self, node: &'a LabeledStatement<'a>) {
    self.declare_statement(&node.body);
  }

  pub fn exec_labeled_statement(&mut self, node: &'a LabeledStatement<'a>) {
    self.pending_labels.push(node);
    self.exec_statement(&node.body);
  }
}
