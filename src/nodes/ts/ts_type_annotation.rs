use crate::{analyzer::Analyzer, ty::Ty};
use oxc::ast::ast::TSTypeAnnotation;

impl<'a> Analyzer<'a> {
  pub fn resolve_type_annotation(&mut self, node: &'a TSTypeAnnotation<'a>) -> Ty<'a> {
    self.resolve_type(&node.type_annotation).unwrap_or(Ty::UnresolvedType(&node.type_annotation))
  }
}
