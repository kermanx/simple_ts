use crate::analyzer::Analyzer;
use oxc::ast::ast::SwitchStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_switch_statement(&mut self, node: &'a SwitchStatement<'a>) {
    todo!()
  }
}
