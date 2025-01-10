use crate::{analyzer::Analyzer, scope::CfScopeKind};
use oxc::ast::ast::{ForStatement, ForStatementInit};

impl<'a> Analyzer<'a> {
  pub fn exec_for_statement(&mut self, node: &'a ForStatement<'a>) {
    let labels = self.take_labels();

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
      let test = self.exec_expression(test);
      if test.test_truthy() == Some(false) {
        return;
      }
    }

    self.push_cf_scope(CfScopeKind::BreakableWithoutLabel, labels.clone(), Some(false));
    self.exec_loop(move |analyzer| {
      if analyzer.cf_scope().must_exited() {
        return;
      }

      analyzer.push_cf_scope(CfScopeKind::Continuable, labels.clone(), None);
      analyzer.exec_statement(&node.body);
      if let Some(update) = &node.update {
        analyzer.exec_expression(update);
      }
      analyzer.pop_cf_scope();

      if let Some(test) = &node.test {
        analyzer.exec_expression(test);
      }
    });
    self.pop_cf_scope();
  }
}
