use crate::{analyzer::Analyzer, ast::AstKind2, r#type::Type};
use oxc::ast::ast::AssignmentTargetMaybeDefault;

#[derive(Debug, Default)]
pub struct WithDefaultData {
  need_init: bool,
}

impl<'a> Analyzer<'a> {
  pub fn exec_assignment_target_maybe_default(
    &mut self,
    node: &'a AssignmentTargetMaybeDefault<'a>,
    value: Type<'a>,
  ) {
    match node {
      AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(node) => {
        let (need_init, value) = self.exec_with_default(&node.init, value);

        let data = self.load_data::<WithDefaultData>(AstKind2::AssignmentTargetWithDefault(node));
        data.need_init |= need_init;

        self.exec_assignment_target_write(&node.binding, value, None);
      }
      _ => self.exec_assignment_target_write(node.to_assignment_target(), value, None),
    }
  }
}
