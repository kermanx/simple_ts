use super::{generic::GenericParam, Ty};
use crate::analyzer::Analyzer;
use oxc::{
  ast::{
    ast::{FormalParameterKind, TSType},
    NONE,
  },
  semantic::SymbolId,
  span::SPAN,
};

#[derive(Debug, Clone)]
pub enum ReturnType<'a> {
  /// function f(): number;
  Simple(Ty<'a>),

  /// function f(a): asserts a is number;
  Assertion(SymbolId, Ty<'a>),

  /// function f(a): a is number;
  Predicate(SymbolId, Ty<'a>),
}

#[derive(Debug, Clone)]
pub struct CallableType<'a, const CTOR: bool> {
  pub type_params: Vec<GenericParam<'a>>,
  pub this_type: Option<Ty<'a>>,
  pub params: Vec<(bool, Ty<'a>)>,
  pub rest_param: Option<Ty<'a>>,
  pub return_type: Ty<'a>,
}

pub type FunctionType<'a> = CallableType<'a, false>;
pub type ConstructorType<'a> = CallableType<'a, true>;

impl<'a> Analyzer<'a> {
  pub fn print_callable_type<const CTOR: bool>(
    &self,
    callable: &CallableType<'a, CTOR>,
  ) -> TSType<'a> {
    self.ast_builder.ts_type_function_type(
      SPAN,
      /* TODO: */ NONE,
      callable.this_type.map(|ty| {
        self.ast_builder.ts_this_parameter(
          SPAN,
          SPAN,
          Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(ty))),
        )
      }),
      self.ast_builder.formal_parameters(
        SPAN,
        FormalParameterKind::Signature,
        {
          let mut items = self.ast_builder.vec();
          for (i, (optional, param)) in callable.params.iter().enumerate() {
            items.push(self.ast_builder.formal_parameter(
              SPAN,
              self.ast_builder.vec(),
              self.ast_builder.binding_pattern(
                self.ast_builder.binding_pattern_kind_binding_identifier(
                  SPAN,
                  &*self.allocator.alloc(format!("a{i}")),
                ),
                Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(*param))),
                *optional,
              ),
              None,
              false,
              false,
            ))
          }
          if let Some(rest) = callable.rest_param {}
          items
        },
        callable.rest_param.map(|ty| {
          self.ast_builder.binding_rest_element(
            SPAN,
            self.ast_builder.binding_pattern(
              self.ast_builder.binding_pattern_kind_binding_identifier(SPAN, "rest"),
              Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(ty))),
              false,
            ),
          )
        }),
      ),
      self.ast_builder.ts_type_annotation(SPAN, self.print_type(callable.return_type)),
    )
  }
}
