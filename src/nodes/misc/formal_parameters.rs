use oxc::ast::ast::FormalParameters;

use crate::{analyzer::Analyzer, ty::ctx::CtxTy};

impl<'a> Analyzer<'a> {
  pub fn exec_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
  ) -> (Option<CtxTy<'a>>, Vec<(bool, CtxTy<'a>)>, Option<CtxTy<'a>>) {
    for param in &node.items {
      self.declare_binding_pattern(&param.pattern, param.pattern.type_annotation.is_some());
    }
    if let Some(rest) = &node.rest {
      self.declare_binding_rest_element(rest, false);
    }

    let mut params = vec![];
    for param in &node.items {
      let inferred = self.init_binding_pattern(&param.pattern, None);
      params.push((
        param.pattern.optional || param.pattern.kind.is_assignment_pattern(),
        self.ctx_ty_from_annotation(&param.pattern.type_annotation, inferred),
      ));
    }

    let rest = if let Some(rest) = &node.rest {
      let inferred = self.init_binding_rest_element(rest, None);
      Some(self.ctx_ty_from_annotation(&rest.argument.type_annotation, inferred))
    } else {
      None
    };

    // TODO: this type
    (None, params, rest)
  }

  pub fn resolve_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
  ) -> (Option<CtxTy<'a>>, Vec<(bool, CtxTy<'a>)>, Option<CtxTy<'a>>) {
    let mut params = vec![];
    for param in &node.items {
      params.push((
        param.pattern.optional,
        self.ctx_ty_from_annotation(&param.pattern.type_annotation, None),
      ));
    }

    let rest = if let Some(rest) = &node.rest {
      Some(self.ctx_ty_from_annotation(&rest.argument.type_annotation, None))
    } else {
      None
    };

    // TODO: this type
    (None, params, rest)
  }
}
