use oxc::{
  ast::ast::{FormalParameters, TSType},
  span::SPAN,
};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
  ) -> (Option<&'a TSType<'a>>, Vec<(bool, &'a TSType<'a>)>, Option<&'a TSType<'a>>) {
    for param in &node.items {
      self.declare_binding_pattern(&param.pattern, param.pattern.type_annotation.is_some());
    }
    if let Some(rest) = &node.rest {
      self.declare_binding_rest_element(rest, false);
    }

    let mut params = vec![];
    for param in &node.items {
      let annotated = param.pattern.type_annotation.as_ref().map(|n| &n.type_annotation);
      let inferred = self.init_binding_pattern(&param.pattern, None).unwrap_or(Ty::Any);
      params.push((
        param.pattern.optional || param.pattern.kind.is_assignment_pattern(),
        annotated.unwrap_or_else(|| self.allocator.alloc(self.serialize_type(inferred))),
      ));
    }

    let rest = if let Some(rest) = &node.rest {
      let annotated = rest.argument.type_annotation.as_ref().map(|n| &n.type_annotation);
      let inferred = self.init_binding_rest_element(rest, None).unwrap_or(Ty::Any);
      Some(annotated.unwrap_or_else(|| self.allocator.alloc(self.serialize_type(inferred))))
    } else {
      None
    };

    // TODO: this type
    (None, params, rest)
  }

  pub fn resolve_formal_parameters(
    &mut self,
    node: &'a FormalParameters<'a>,
  ) -> (Option<&'a TSType<'a>>, Vec<(bool, &'a TSType<'a>)>, Option<&'a TSType<'a>>) {
    let mut params = vec![];
    for param in &node.items {
      params.push((
        param.pattern.optional,
        param.pattern.type_annotation.as_ref().map_or_else(
          || &*self.allocator.alloc(self.ast_builder.ts_type_any_keyword(SPAN)),
          |n| &n.type_annotation,
        ),
      ));
    }

    let rest = if let Some(rest) = &node.rest {
      Some(rest.argument.type_annotation.as_ref().map_or_else(
        || &*self.allocator.alloc(self.ast_builder.ts_type_any_keyword(SPAN)),
        |n| &n.type_annotation,
      ))
    } else {
      None
    };

    // TODO: this type
    (None, params, rest)
  }
}
