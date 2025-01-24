use crate::{
  ty::{r#match::MatchResult, Ty},
  Analyzer,
};
use oxc::ast::ast::TSConditionalType;

impl<'a> Analyzer<'a> {
  pub fn resolve_conditional_type(&mut self, node: &'a TSConditionalType<'a>) -> Ty<'a> {
    let target = self.resolve_type(&node.check_type);
    let pattern = self.resolve_type(&node.extends_type);

    let mut matched_no_infer = None;
    let mut unmatched = None;
    let mut results = Vec::new();

    for m in self.match_types_with_dispatch(target, pattern) {
      match m {
        MatchResult::Error => return Ty::Error,
        MatchResult::Matched => {
          results.push(*matched_no_infer.get_or_insert_with(|| {
            let old_generics = self.take_generics();
            let infer_declarations = self.semantic.scopes().get_bindings(node.scope_id());
            for symbol in infer_declarations.values() {
              self.type_scopes.insert(*symbol, Ty::Unknown);
            }
            let result = self.resolve_type(&node.true_type);
            self.restore_generics(old_generics);
            result
          }));
        }
        MatchResult::Inferred(inferred) => {
          let old_generics = self.replace_generics(Box::new(inferred));
          let infer_declarations = self.semantic.scopes().get_bindings(node.scope_id());
          for symbol in infer_declarations.values() {
            self.type_scopes.entry(*symbol).or_insert(Ty::Unknown);
          }
          results.push(self.resolve_type(&node.true_type));
          self.restore_generics(old_generics);
        }
        MatchResult::Unmatched => {
          results.push(*unmatched.get_or_insert_with(|| self.resolve_type(&node.false_type)));
        }
      }
    }

    self.into_union(results).unwrap()
  }
}
