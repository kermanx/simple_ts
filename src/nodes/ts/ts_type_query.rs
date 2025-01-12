use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::TSTypeQuery;

impl<'a> Analyzer<'a> {
  pub fn resolve_type_query(&mut self, node: &'a TSTypeQuery<'a>) -> Type<'a> {
    let type_name = self.resolve_type_name(&node.type_name);
    Type::TypeQuery(type_name)
  }
}
