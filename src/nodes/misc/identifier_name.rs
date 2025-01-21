use oxc::ast::ast::IdentifierName;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_identifier_name(&mut self, node: &'a IdentifierName<'a>) -> Ty<'a> {
    Ty::StringLiteral(&node.name)
  }
}
