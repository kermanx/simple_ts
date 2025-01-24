use oxc::ast::ast::TSTypeAliasDeclaration;

use crate::{
  ty::{generic::GenericType, Ty},
  Analyzer,
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_type_alias(&mut self, node: &'a TSTypeAliasDeclaration<'a>) {
    let symbol_id = node.id.symbol_id();
    let ty = if let Some(type_parameters) = &node.type_parameters {
      let params = self.resolve_type_parameter_declaration(type_parameters);
      Ty::Generic(self.allocator.alloc(GenericType {
        name: &node.id.name,
        params,
        body: &node.type_annotation,
      }))
    } else {
      self.resolve_type(&node.type_annotation)
    };
    self.types.insert(symbol_id, ty);
    self.accumulate_type(&node.id, ty);
  }

  pub fn init_ts_type_alias(&mut self, _node: &'a TSTypeAliasDeclaration<'a>) {
    // Do nothing
  }
}
