use oxc::ast::ast::ReturnStatement;

use crate::{analyzer::Analyzer, scope::call::CallScopeReturnType, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_return_statement(&mut self, node: &'a ReturnStatement) {
    let call_scope = self.call_scopes.last().unwrap();
    match &call_scope.ret {
      CallScopeReturnType::Annotated(ty) => {
        if let Some(argument) = &node.argument {
          self.exec_expression(argument, Some(*ty));
        }
      }
      CallScopeReturnType::Inferred(_) => {
        let ty = if let Some(argument) = &node.argument {
          self.exec_expression(argument, None)
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
