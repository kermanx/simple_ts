use crate::{analyzer::Analyzer, r#type::Type};
use oxc::{ast::ast::PrivateIdentifier, span::Atom};

impl<'a> Analyzer<'a> {
  pub fn exec_private_identifier(&mut self, node: &'a PrivateIdentifier<'a>) -> Type<'a> {
    // FIXME: Not good
    Type::StringLiteral(
      self.allocator.alloc(Atom::from(self.escape_private_identifier_name(node.name.as_str()))),
    )
  }
}
