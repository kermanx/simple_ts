use crate::analyzer::Analyzer;
use oxc::ast::ast::ContinueStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_continue_statement(&mut self, node: &'a ContinueStatement<'a>) {
    let label = node.label.as_ref().map(|label| label.name.as_str());
    self.continue_to_label(label);
  }
}
