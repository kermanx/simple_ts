use oxc::ast::ast::{FunctionBody, Statement};

use crate::{
  analyzer::Analyzer,
  scope::{
    call::{CallScope, CallScopeReturnType},
    cf::CfScopeKind,
  },
  ty::Ty,
};

impl<'a> Analyzer<'a> {
  pub fn exec_function_body(
    &mut self,
    node: &'a FunctionBody<'a>,
    is_async: bool,
    is_generator: bool,
    this: Option<Ty<'a>>,
    annotated_ret: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let body_scope = self.push_scope(CfScopeKind::Function);
    self.call_scopes.push(CallScope::new(
      body_scope,
      is_async,
      is_generator,
      this.unwrap_or(Ty::Any),
      annotated_ret,
    ));

    self.exec_statement_vec(&node.statements);

    self.pop_scope();
    match self.call_scopes.pop().unwrap().ret {
      CallScopeReturnType::Annotated(ty) => ty,
      CallScopeReturnType::Inferred(mut acc) => acc.to_ty().unwrap_or(Ty::Void),
    }
  }

  pub fn exec_function_expression_body(
    &mut self,
    node: &'a FunctionBody<'a>,
    is_async: bool,
    annotated_ret: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let [Statement::ExpressionStatement(expr)] = node.statements.as_slice() else {
      unreachable!();
    };
    self.push_scope(CfScopeKind::Function);
    let value = self.exec_expression(&expr.expression, annotated_ret);
    self.pop_scope();
    value
  }
}
