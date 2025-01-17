use crate::{analyzer::Analyzer, scope::call::CallScopeReturnType, ty::Ty};
use oxc::ast::ast::ReturnStatement;

impl<'a> Analyzer<'a> {
  pub fn exec_return_statement(&mut self, node: &'a ReturnStatement) {
    let call_scope = self.call_scopes.last().unwrap();
    match &call_scope.ret {
      CallScopeReturnType::Annotated(ty) => {
        // FIXME: backward inference
        if let Some(argument) = &node.argument {
          self.exec_expression(argument);
        }
      }
      CallScopeReturnType::Inferred(_) => {
        let ty = if let Some(argument) = &node.argument {
          self.exec_expression(argument)
        } else {
          Ty::Undefined
        };
        let call_scope = self.call_scopes.last_mut().unwrap();
        let CallScopeReturnType::Inferred(acc) = &mut call_scope.ret else { unreachable!() };
        acc.add(ty, self.allocator);
      }
    }
  }
}
