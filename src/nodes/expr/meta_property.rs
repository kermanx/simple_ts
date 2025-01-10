use crate::{analyzer::Analyzer, entity::Entity};
use oxc::ast::ast::MetaProperty;

impl<'a> Analyzer<'a> {
  pub fn exec_meta_property(&mut self, node: &'a MetaProperty<'a>) -> Entity<'a> {
    let meta = node.meta.name.as_str();
    let property = node.property.name.as_str();

    if meta == "import" && property == "meta" {
      self.builtins.import_meta
    } else {
      self.factory.unknown
    }
  }
}
