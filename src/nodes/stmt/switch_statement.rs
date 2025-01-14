use crate::{analyzer::Analyzer, ast::AstKind2};
use oxc::ast::ast::SwitchStatement;
use rustc_hash::FxHashSet;

impl<'a> Analyzer<'a> {
  pub fn exec_switch_statement(&mut self, node: &'a SwitchStatement<'a>) {
    // 1. discriminant
    let discriminant = self.exec_expression(&node.discriminant);

    // 2. tests
    let mut default_case = None;
    let mut maybe_default_case = Some(true);
    let mut test_results = vec![];
    let mut indeterminate = false;
    for (index, case) in node.cases.iter().enumerate() {
      if let Some(test) = &case.test {
        let test_val = self.exec_expression(test);

        // TODO: Support mangling
        let test_result = self.entity_op.strict_eq(self, discriminant, test_val);
        test_results.push(test_result);

        if test_result != Some(false) {
          data.need_test.insert(index);
        }

        match test_result {
          Some(true) => {
            maybe_default_case = Some(false);
            break;
          }
          Some(false) => {}
          None => {
            // data.need_test.insert(index);
            maybe_default_case = None;
            if !indeterminate {
              indeterminate = true;
              self.push_indeterminate_cf_scope();
            }
          }
        }
      } else {
        default_case = Some(index);
        test_results.push(/* Updated later */ None);
      }
    }
    if indeterminate {
      self.pop_cf_scope();
    }

    // Patch default case
    if let Some(default_case) = default_case {
      test_results[default_case] = maybe_default_case;
      if maybe_default_case != Some(false) {
        data.need_test.insert(default_case);
      }
    }

    // 3. consequent
    self.push_cf_scope(CfScopeKind::BreakableWithoutLabel, labels, Some(false));
    let mut entered = Some(false);
    for (index, case) in node.cases.iter().enumerate() {
      if self.cf_scope().must_exited() {
        break;
      }

      let test_result = test_results.get(index).unwrap_or(&Some(false));

      entered = match (test_result, entered) {
        (Some(true), Some(true)) => unreachable!(),
        (Some(true), _) => Some(true),
        (Some(false), prev) => prev,
        (None, Some(true)) => Some(true),
        (None, _) => None,
      };

      if entered != Some(false) {
        data.need_consequent.insert(index);

        if entered.is_none() {
          self.push_indeterminate_cf_scope();
        }
        self.exec_statement_vec(&case.consequent);
        if entered.is_none() {
          self.pop_cf_scope();
        }
      }
    }

    self.pop_cf_scope();
  }
}
