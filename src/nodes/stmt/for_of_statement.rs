use crate::{analyzer::Analyzer, scope::CfScopeKind};
use oxc::ast::ast::ForOfStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_for_of_statement(&mut self, node: &'a ForOfStatement<'a>) {
    let labels = self.take_labels();

    let right = self.exec_expression(&node.right);
    let right = if node.r#await {
      right.unknown_mutation(self);
      self.factory.unknown
    } else {
      right
    };

    self.declare_for_statement_left(&node.left);

    let Some(iterated) = right.iterate_result_union(self) else {
      return;
    };

    self.push_cf_scope(CfScopeKind::BreakableWithoutLabel, labels.clone(), Some(false));
    self.exec_loop(move |analyzer| {
      analyzer.declare_for_statement_left(&node.left);
      analyzer.init_for_statement_left(&node.left, iterated);

      analyzer.push_cf_scope(CfScopeKind::Continuable, labels.clone(), None);
      analyzer.exec_statement(&node.body);
      analyzer.pop_cf_scope();
    });
    self.pop_cf_scope();
  }
}
