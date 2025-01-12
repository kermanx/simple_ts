use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TSTypeAnnotation;

impl<'a> Analyzer<'a> {
  pub fn resolve_type_annotation(&mut self, node: &'a TSTypeAnnotation<'a>) -> Type<'a> {
    self.resolve_type(&node.type_annotation).unwrap_or(Type::Unresolved(&node.type_annotation))
  }
}
