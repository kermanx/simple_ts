use crate::{analyzer::Analyzer, r#type::TypeofResult, scope::CfScopeKind};
use oxc::ast::ast::ForInStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_for_in_statement(&mut self, node: &'a ForInStatement<'a>) {
    let labels = self.take_labels();
    let right = self.exec_expression(&node.right);

    // FIXME: enumerate keys!
    right.unknown_mutation(self);

    let types_have_no_keys: TypeofResult = TypeofResult::Undefined
      | TypeofResult::Boolean
      | TypeofResult::Number
      | TypeofResult::String
      | TypeofResult::Symbol;

    // TODO: empty object, simple function, array
    if (right.test_typeof() & !types_have_no_keys) == TypeofResult::_None
      || right.test_nullish() == Some(true)
    {
      return;
    }

    self.declare_for_statement_left(&node.left);

    self.push_cf_scope(CfScopeKind::BreakableWithoutLabel, labels.clone(), Some(false));
    self.exec_loop(move |analyzer| {
      analyzer.declare_for_statement_left(&node.left);
      analyzer.init_for_statement_left(&node.left, analyzer.factory.string);

      analyzer.push_cf_scope(CfScopeKind::Continuable, labels.clone(), None);
      analyzer.exec_statement(&node.body);
      analyzer.pop_cf_scope();
    });
    self.pop_cf_scope();
  }
}
