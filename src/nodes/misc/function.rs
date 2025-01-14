use crate::{
  analyzer::Analyzer,
  ast::{AstKind2, DeclarationKind},
  r#type::Type,
  utils::{CalleeInfo, CalleeNode},
};
use oxc::{
  ast::ast::{Function, FunctionType},
  semantic::ScopeId,
};
use std::rc::Rc;

impl<'a> Analyzer<'a> {
  pub fn exec_function(&mut self, node: &'a Function<'a>) -> Type<'a> {
    self.new_function(CalleeNode::Function(node))
  }

  pub fn declare_function(&mut self, node: &'a Function<'a>, exporting: bool) {
    let entity = self.exec_function(node);

    let symbol = node.id.as_ref().unwrap().symbol_id.get().unwrap();
    self.declare_symbol(
      symbol,
      AstKind2::Function(node),
      exporting,
      DeclarationKind::Function,
      Some(entity),
    );
  }

  pub fn call_function(
    &mut self,
    fn_entity: Type<'a>,
    callee: CalleeInfo<'a>,
    node: &'a Function<'a>,
    variable_scopes: Rc<Vec<ScopeId>>,
    this: Type<'a>,
    args: Type<'a>,
    consume: bool,
  ) -> Type<'a> {
    let runner: Box<dyn Fn(&mut Analyzer<'a>) -> Type<'a> + 'a> =
      Box::new(move |analyzer: &mut Analyzer<'a>| {
        analyzer.push_call_scope(variable_scopes.as_ref().clone(), node.r#async, node.generator);

        let variable_scope = analyzer.variable_scope_mut();
        variable_scope.this = Some(this);
        variable_scope.arguments = Some((args, vec![ /* later filled by formal parameters */]));

        let declare_in_body = node.r#type == FunctionType::FunctionExpression && node.id.is_some();
        if declare_in_body {
          let symbol = node.id.as_ref().unwrap().symbol_id.get().unwrap();
          analyzer.declare_symbol(
            symbol,
            callee.into_node(),
            false,
            DeclarationKind::NamedFunctionInBody,
            Some(fn_entity),
          );
        }

        analyzer.exec_formal_parameters(&node.params, args, DeclarationKind::FunctionParameter);
        analyzer.exec_function_body(node.body.as_ref().unwrap());

        if consume {
          analyzer.consume_return_values();
        }

        analyzer.pop_call_scope()
      });

    if node.r#async || node.generator {
      // Too complex to analyze the control flow, thus run exhaustively
      self.exec_async_or_generator_fn(move |analyzer| {
        runner(analyzer).unknown_mutation(analyzer);
      });
      self.factory.unknown
    } else {
      runner(self)
    }
  }
}
