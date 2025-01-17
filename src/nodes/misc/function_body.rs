use crate::{
  analyzer::Analyzer,
  scope::{
    call::{CallScope, CallScopeReturnType},
    cf::CfScopeKind,
  },
  ty::Ty,
};
use oxc::ast::ast::{FunctionBody, Statement};

impl<'a> Analyzer<'a> {
  pub fn exec_function_body(
    &mut self,
    node: &'a FunctionBody<'a>,
    is_async: bool,
    is_generator: bool,
    this: Ty<'a>,
    annotated_ret: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let body_scope = self.push_scope(CfScopeKind::Function);
    self.call_scopes.push(CallScope::new(body_scope, is_async, is_generator, this, annotated_ret));

    self.exec_statement_vec(&node.statements);

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
    let value = if let Some(annotated_ret) = annotated_ret {
      todo!("backward inference")
    } else {
      self.exec_expression(&expr.expression)
    };
    self.pop_scope();
    value
  }
}
