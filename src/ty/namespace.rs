use oxc::ast::ast::TSType;
use rustc_hash::FxHashMap;

use super::Ty;
use crate::analyzer::Analyzer;

#[derive(Debug, Clone)]
pub struct NamespaceType<'a> {
  pub members: FxHashMap<&'a str, Ty<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn print_namespace_type(&self, namespace: &NamespaceType<'a>) -> TSType<'a> {
    todo!()
  }
}
