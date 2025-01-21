use oxc::ast::ast::{ForStatement, ForStatementInit};

use crate::analyzer::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn exec_for_statement(&mut self, node: &'a ForStatement<'a>) {
    if let Some(init) = &node.init {
      match init {
        ForStatementInit::VariableDeclaration(node) => {
          self.declare_variable_declaration(node);
          self.init_variable_declaration(node, None);
        }
        node => {
          self.exec_expression(node.to_expression(), None);
        }
      }
    }

    self.push_loop_scope();

    if let Some(test) = &node.test {
      self.exec_expression(test, None);
      // CHECKER
    }

    if let Some(update) = &node.update {
      self.exec_expression(update, None);
    }

    self.exec_statement(&node.body);
    self.pop_scope();
  }
}
