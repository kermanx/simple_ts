use crate::{
  analyzer::Analyzer,
  ast::DeclarationKind,
  entity::Entity,
  utils::{CalleeInfo, CalleeNode},
};
use oxc::{ast::ast::ArrowFunctionExpression, semantic::ScopeId};
use std::rc::Rc;

impl<'a> Analyzer<'a> {
  pub fn exec_arrow_function_expression(
    &mut self,
    node: &'a ArrowFunctionExpression<'a>,
  ) -> Entity<'a> {
    self.new_function(CalleeNode::ArrowFunctionExpression(node))
  }

  pub fn call_arrow_function_expression(
    &mut self,
    callee: CalleeInfo<'a>,
    node: &'a ArrowFunctionExpression<'a>,
    variable_scopes: Rc<Vec<ScopeId>>,
    args: Entity<'a>,
    consume: bool,
  ) -> Entity<'a> {
    self.push_call_scope(callee, variable_scopes.as_ref().clone(), node.r#async, false, consume);

    self.exec_formal_parameters(&node.params, args, DeclarationKind::ArrowFunctionParameter);
    if node.expression {
      self.exec_function_expression_body(&node.body);
    } else {
      self.exec_function_body(&node.body);
    }

    if consume {
      self.consume_return_values();
    }

    self.pop_call_scope()
  }
}
