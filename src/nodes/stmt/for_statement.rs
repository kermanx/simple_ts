use crate::analyzer::Analyzer;
use oxc::ast::ast::{ForStatement, ForStatementInit};

impl<'a> Analyzer<'a> {
  pub fn exec_for_statement(&mut self, node: &'a ForStatement<'a>) {
    self.push_variable_scope();

    if let Some(init) = &node.init {
      match init {
        ForStatementInit::VariableDeclaration(node) => {
          self.declare_variable_declaration(node, false);
          self.init_variable_declaration(node, None);
        }
        node => {
          self.exec_expression(node.to_expression());
        }
      }
    }

    if let Some(test) = &node.test {
      self.push_indeterminate_cf_scope();
      self.exec_expression(test);
      // CHECKER
      self.pop_cf_scope();
    }

    if let Some(update) = &node.update {
      self.push_indeterminate_cf_scope();
      self.exec_expression(update);
      self.pop_cf_scope();
    }

    self.push_loop_cf_scope();
    self.exec_statement(&node.body);
    self.pop_cf_scope();

    self.pop_variable_scope();
  }
}
