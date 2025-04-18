use crate::ty::{Ty, accumulator::TypeAccumulator};

use super::runtime::RuntimeScopeId;

pub enum CallScopeReturnType<'a> {
  Annotated(Ty<'a>),
  Inferred(TypeAccumulator<'a>),
}

pub struct CallScope<'a> {
  pub body_scope: RuntimeScopeId,

  pub is_async: bool,
  pub is_generator: bool,

  pub this: Ty<'a>,
  pub ret: CallScopeReturnType<'a>,

  #[cfg(feature = "flame")]
  pub scope_guard: flame::SpanGuard,
}

impl<'a> CallScope<'a> {
  pub fn new(
    body_scope: RuntimeScopeId,
    is_async: bool,
    is_generator: bool,
    this: Ty<'a>,
    annotated_ret: Option<Ty<'a>>,
  ) -> Self {
    CallScope {
      body_scope,

      is_async,
      is_generator,

      this,
      ret: if let Some(annotated_ret) = annotated_ret {
        CallScopeReturnType::Annotated(annotated_ret)
      } else {
        CallScopeReturnType::Inferred(Default::default())
      },

      #[cfg(feature = "flame")]
      scope_guard: flame::start_guard(callee.debug_name.to_string()),
    }
  }
}
