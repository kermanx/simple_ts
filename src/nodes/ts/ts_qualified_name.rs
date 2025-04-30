use oxc::ast::ast::TSQualifiedName;

use crate::{
  Analyzer,
  ty::{Ty, namespace::Ns},
};

impl<'a> Analyzer<'a> {
  pub fn resolve_qualified_name_ty(&mut self, node: &'a TSQualifiedName<'a>) -> Ty<'a> {
    if let Some(left) = self.resolve_type_name_ns(&node.left) {
      left.borrow().types.get(&node.right.name).copied().unwrap_or(Ty::Error)
    } else {
      Ty::Error
    }
  }

  pub fn resolve_qualified_name_ns(&mut self, node: &'a TSQualifiedName<'a>) -> Option<&'a Ns<'a>> {
    let left = self.resolve_type_name_ns(&node.left)?;
    left.borrow().children.get(&node.right.name).copied()
  }
}
