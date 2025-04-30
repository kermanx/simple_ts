use oxc::ast::ast::{TSQualifiedName, TSTypeName};

use crate::{
  Analyzer,
  ty::{Ty, namespace::Ns, property_key::PropertyKeyType},
};

#[derive(Debug, Clone, Copy)]
pub enum NsOrTy<'a> {
  Ns(&'a Ns<'a>),
  Ty(Ty<'a>),
}

impl<'a> Analyzer<'a> {
  pub fn resolve_qualified_name(
    &mut self,
    node: &'a TSQualifiedName<'a>,
    need_ns: bool,
  ) -> NsOrTy<'a> {
    let left = match &node.left {
      TSTypeName::IdentifierReference(node) => {
        let reference = self.semantic.scoping().get_reference(node.reference_id());
        if let Some(symbol_id) = reference.symbol_id() {
          if let Some(namespace) = self.namespaces.get(&symbol_id) {
            NsOrTy::Ns(*namespace)
          } else {
            todo!()
          }
        } else {
          NsOrTy::Ty(Ty::Any)
        }
      }
      TSTypeName::QualifiedName(node) => self.resolve_qualified_name(node, true),
    };
    match left {
      NsOrTy::Ns(ns) => {
        if need_ns {
          if let Some(child) = ns.borrow().children.get(&node.right.name) {
            NsOrTy::Ns(child)
          } else {
            NsOrTy::Ty(Ty::Error)
          }
        } else {
          NsOrTy::Ty(
            ns.borrow().record.get_property(PropertyKeyType::StringLiteral(&node.right.name)),
          )
        }
      }
      NsOrTy::Ty(_) => NsOrTy::Ty(Ty::Error),
    }
  }
}
