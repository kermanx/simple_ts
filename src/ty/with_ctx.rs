use crate::{scope::r#type::TypeScopeId, Analyzer};

use super::Ty;

#[derive(Debug, Clone, Copy)]
pub struct WithCtxType<'a> {
  pub scope: TypeScopeId,
  pub ty: Ty<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn wrap_with_ctx(&self, ty: Ty<'a>) -> Ty<'a> {
    let scope = self.type_scopes.top();
    Ty::WithCtx(self.allocator.alloc(WithCtxType { scope, ty }))
  }

  pub fn resolve_with_ctx<R>(
    &mut self,
    with_ctx: &WithCtxType<'a>,
    f: impl FnOnce(&mut Self, Ty<'a>) -> R,
  ) -> R {
    let old_top = self.type_scopes.replace_top(with_ctx.scope);
    let result = f(self, with_ctx.ty);
    self.type_scopes.replace_top(old_top);
    result
  }
}
