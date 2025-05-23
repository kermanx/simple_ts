use oxc::ast::ast::{TSType, TSTypeAliasDeclaration};

use crate::{
  Analyzer,
  ty::{Ty, generic::GenericType, intrinsics::IntrinsicType},
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_type_alias(&mut self, node: &'a TSTypeAliasDeclaration<'a>) {
    let ty = if let Some(type_parameters) = &node.type_parameters {
      let params = self.resolve_type_parameter_declaration(type_parameters);
      if matches!(node.type_annotation, TSType::TSIntrinsicKeyword(_)) {
        Ty::Intrinsic(IntrinsicType::from_name(&node.id.name))
      } else {
        Ty::Generic(self.allocator.alloc(GenericType {
          name: &node.id.name,
          params,
          body: self.ctx_ty_from_ts_type(&node.type_annotation),
        }))
      }
    } else {
      self.resolve_type(&node.type_annotation)
    };
    self.declare_type_identifier(
      &node.id, false, // FIXME: export
      ty,
    );
  }

  pub fn init_ts_type_alias(&mut self, _node: &'a TSTypeAliasDeclaration<'a>) {
    // Do nothing
  }
}
