use crate::{analyzer::Analyzer, ast::AstKind2, scope::CfScopeKind};
use oxc::ast::ast::DoWhileStatement;

#[derive(Debug, Default, Clone)]
pub struct Data {
  need_test: bool,
  need_loop: bool,
}

impl<'a> Analyzer<'a> {
  pub fn exec_do_while_statement(&mut self, node: &'a DoWhileStatement<'a>) {
    let labels = self.take_labels();
    self.push_cf_scope(CfScopeKind::BreakableWithoutLabel, labels.clone(), Some(false));

    // Execute the first round.
    self.push_cf_scope(CfScopeKind::Continuable, labels.clone(), Some(false));
    self.exec_statement(&node.body);
    self.pop_cf_scope();

    if self.cf_scope().must_exited() {
      self.pop_cf_scope();
      return;
    }

    let data = self.load_data::<Data>(AstKind2::DoWhileStatement(node));
    data.need_test = true;
    let test = self.exec_expression(&node.test);

    // The rest is the same as while statement.
    if test.test_truthy() == Some(false) {
      self.pop_cf_scope();
      return;
    }
    test.consume(self);

    data.need_loop = true;

    self.exec_loop(move |analyzer| {
      analyzer.push_cf_scope(CfScopeKind::Continuable, labels.clone(), None);

      analyzer.exec_statement(&node.body);
      analyzer.exec_expression(&node.test).consume(analyzer);

      analyzer.pop_cf_scope();
    });

    self.pop_cf_scope();
  }
}
