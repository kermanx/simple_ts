use oxc::ast::ast::BreakStatement;

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_break_statement(&mut self, node: &'a BreakStatement<'a>) {
    let label = node.label.as_ref().map(|label| &label.name);
    self.break_to_label(label);
  }
}
