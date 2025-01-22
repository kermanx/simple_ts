use oxc::ast::ast::FormalParameters;

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
  ) -> (Option<Ty<'a>>, Vec<(bool, Ty<'a>)>, Option<Ty<'a>>) {
    for param in &node.items {
      self.declare_binding_pattern(&param.pattern, param.pattern.type_annotation.is_some());
    }

    let mut params = vec![];
    for param in &node.items {
      params.push((
        param.pattern.optional,
        self.init_binding_pattern(&param.pattern, None).unwrap_or(Ty::Any),
      ));
    }

    let rest = if let Some(rest) = &node.rest {
      self.declare_binding_rest_element(rest, false);
      Some(self.init_binding_rest_element(rest, None).unwrap_or(Ty::Any))
    } else {
      None
    };

    // TODO: this type
    (None, params, rest)
  }

  pub fn resolve_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
  ) -> (Option<Ty<'a>>, Vec<(bool, Ty<'a>)>, Option<Ty<'a>>) {
    let mut params = vec![];
    for param in &node.items {
      params.push((
        param.pattern.optional,
        param
          .pattern
          .type_annotation
          .as_ref()
          .map_or(Ty::Any, |annotation| self.resolve_type_annotation(annotation)),
      ));
    }

    let rest = if let Some(rest) = &node.rest {
      Some(
        rest
          .argument
          .type_annotation
          .as_ref()
          .map_or(Ty::Any, |annotation| self.resolve_type_annotation(annotation)),
      )
    } else {
      None
    };

    // TODO: this type
    (None, params, rest)
  }
}
