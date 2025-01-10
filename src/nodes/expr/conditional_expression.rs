use crate::{analyzer::Analyzer, entity::Entity, scope::CfScopeKind};
use oxc::ast::ast::ConditionalExpression;

impl<'a> Analyzer<'a> {
  pub fn exec_conditional_expression(&mut self, node: &'a ConditionalExpression<'a>) -> Entity<'a> {
    let test = self.exec_expression(&node.test);

    let (maybe_true, maybe_false) = match test.test_truthy() {
      Some(true) => (true, false),
      Some(false) => (false, true),
      None => (true, true),
    };

    let exec_consequent = move |analyzer: &mut Analyzer<'a>| {
      analyzer.push_cf_scope(
        CfScopeKind::ConditionalExprBranch,
        None,
        if maybe_false { None } else { Some(false) },
      );
      let value = analyzer.exec_expression(&node.consequent);
      analyzer.pop_cf_scope();
      value
    };

    let exec_alternate = move |analyzer: &mut Analyzer<'a>| {
      analyzer.push_cf_scope(
        CfScopeKind::ConditionalExprBranch,
        None,
        if maybe_true { None } else { Some(false) },
      );
      let value = analyzer.exec_expression(&node.alternate);
      analyzer.pop_cf_scope();
      value
    };

    match (maybe_true, maybe_false) {
      (true, false) => exec_consequent(self),
      (false, true) => exec_alternate(self),
      (true, true) => {
        let v1 = exec_consequent(self);
        let v2 = exec_alternate(self);
        self.factory.union((v1, v2))
      }
      _ => unreachable!("Conditional expression should have at least one possible branch"),
    }
  }
}
