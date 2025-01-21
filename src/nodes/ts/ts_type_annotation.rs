use oxc::ast::ast::TSTypeAnnotation;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn resolve_type_annotation(&mut self, node: &'a TSTypeAnnotation<'a>) -> Ty<'a> {
    self.resolve_type(&node.type_annotation)
  }
}
