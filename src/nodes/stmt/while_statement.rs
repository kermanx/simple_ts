use crate::{analyzer::Analyzer, scope::CfScopeKind};
use oxc::ast::ast::WhileStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_while_statement(&mut self, node: &'a WhileStatement<'a>) {
    // This may be indeterminate. However, we can't know it until we execute the test.
    // And there should be no same level break/continue statement in test.
    // `a: while(() => { break a }) { }` is illegal.
    let test = self.exec_expression(&node.test);

    if test.test_truthy() == Some(false) {
      return;
    }

    self.push_cf_scope(CfScopeKind::BreakableWithoutLabel, labels.clone(), Some(false));
    self.exec_loop(move |analyzer| {
      analyzer.push_cf_scope(CfScopeKind::Continuable, labels.clone(), None);

      analyzer.exec_statement(&node.body);
      analyzer.exec_expression(&node.test).unknown_mutation(analyzer);

      analyzer.pop_cf_scope();

      analyzer.exec_expression(&node.test);
    });
    self.pop_cf_scope();
  }
}
