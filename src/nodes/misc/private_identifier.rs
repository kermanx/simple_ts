use oxc::{ast::ast::PrivateIdentifier, span::Atom};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_private_identifier(&mut self, node: &'a PrivateIdentifier<'a>) -> Ty<'a> {
    // FIXME: Not good
    Ty::StringLiteral(
      self.allocator.alloc(Atom::from(self.escape_private_identifier_name(node.name.as_str()))),
    )
  }
}
