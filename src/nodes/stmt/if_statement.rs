use crate::analyzer::Analyzer;
use oxc::ast::ast::IfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_if_statement(&mut self, node: &'a IfStatement) {
    self.exec_expression(&node.test);

    self.push_variable_scope();
    self.exec_statement(&node.consequent);
    let scope_1 = self.pop_variable_scope();

    if let Some(alternate) = &node.alternate {
      self.push_variable_scope();
      self.exec_statement(alternate);
      let scope_2 = self.pop_variable_scope();
      self.apply_shadows([scope_1, scope_2], true);
    } else {
      self.apply_shadows([scope_1], false);
    }
  }
}
