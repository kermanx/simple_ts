use oxc::ast::ast::ForStatementLeft;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn declare_for_statement_left(&mut self, node: &'a ForStatementLeft<'a>) {
    if let ForStatementLeft::VariableDeclaration(node) = node {
      self.declare_variable_declaration(node);
    }
  }

  pub fn init_for_statement_left(&mut self, node: &'a ForStatementLeft<'a>, init: Ty<'a>) {
    match node {
      ForStatementLeft::VariableDeclaration(node) => {
        self.init_variable_declaration(node, Some(init));
      }
      _ => self.exec_assignment_target_write(node.to_assignment_target(), init, None),
    }
  }
}
