use super::{
  consumed_object, Entity, EntityTrait, EnumeratedProperties, IteratedElements, ObjectEntity,
  TypeofResult,
};
use crate::{
  analyzer::Analyzer,
  use_consumed_flag,
  utils::{CalleeInfo, CalleeNode},
};
use oxc::semantic::ScopeId;
use std::{cell::Cell, rc::Rc};

#[derive(Debug)]
pub struct FunctionEntity<'a> {
  consumed: Rc<Cell<bool>>,
  body_consumed: Rc<Cell<bool>>,
  pub callee: CalleeInfo<'a>,
  pub variable_scope_stack: Rc<Vec<ScopeId>>,
  pub object: &'a ObjectEntity<'a>,
}

impl<'a> EntityTrait<'a> for FunctionEntity<'a> {
  fn consume(&'a self, analyzer: &mut Analyzer<'a>) {
    use_consumed_flag!(self);

    self.consume_body(analyzer);

    self.object.consume(analyzer);
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    self.object.get_property(analyzer, key)
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    self.object.set_property(analyzer, key, value);
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    self.object.delete_property(analyzer, key);
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    consumed_object::enumerate_properties(self, analyzer)
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::call(self, analyzer, this, args);
    }

    let mut recursion_depth = 0usize;
    for scope in analyzer.scope_context.call.iter().rev() {
      if scope.callee.node == self.callee.node {
        recursion_depth += 1;
        if recursion_depth >= analyzer.config.max_recursion_depth {
          self.consume_body(analyzer);
          return consumed_object::call(self, analyzer, this, args);
        }
      }
    }

    self.call_impl(analyzer, this, args, false)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, props: Entity<'a>) -> Entity<'a> {
    self.call(analyzer, analyzer.factory.unknown, analyzer.factory.arguments(vec![(false, props)]))
  }

  fn r#await(&'a self, analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::r#await(analyzer);
    }
    self
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    self.consume(analyzer);
    consumed_object::iterate(analyzer)
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string("function")
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_to_string(analyzer);
    }
    analyzer.factory.unknown_string
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_to_numeric(analyzer);
    }
    analyzer.factory.nan
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.boolean(true)
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.get_to_string(analyzer)
  }

  fn get_to_jsx_child(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      analyzer.factory.unknown
    } else {
      // TODO: analyzer.thrown_builtin_error("Functions are not valid JSX children");
      analyzer.factory.string("")
    }
  }

  fn test_typeof(&self) -> TypeofResult {
    TypeofResult::Function
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(true)
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> FunctionEntity<'a> {
  pub fn call_impl(
    &'a self,
    analyzer: &mut Analyzer<'a>,

    this: Entity<'a>,
    args: Entity<'a>,
    consume: bool,
  ) -> Entity<'a> {
    let variable_scopes = self.variable_scope_stack.clone();
    let ret_val = match self.callee.node {
      CalleeNode::Function(node) => {
        analyzer.call_function(self, self.callee, node, variable_scopes, this, args, consume)
      }
      CalleeNode::ArrowFunctionExpression(node) => {
        analyzer.call_arrow_function_expression(self.callee, node, variable_scopes, args, consume)
      }
      _ => unreachable!(),
    };
    ret_val
  }

  pub fn consume_body(&'a self, analyzer: &mut Analyzer<'a>) {
    if self.body_consumed.replace(true) {
      return;
    }

    analyzer.exec_consumed_fn("consume_fn", move |analyzer| {
      self.call_impl(analyzer, analyzer.factory.unknown, analyzer.factory.unknown, true)
    });
  }
}

impl<'a> Analyzer<'a> {
  pub fn new_function(&mut self, node: CalleeNode<'a>) -> Entity<'a> {
    let function = self.factory.alloc(FunctionEntity {
      consumed: Rc::new(Cell::new(false)),
      body_consumed: Rc::new(Cell::new(false)),
      callee: self.new_callee_info(node),
      variable_scope_stack: Rc::new(self.scope_context.variable.stack.clone()),
      object: self.new_function_object(),
    });

    let mut created_in_self = false;
    for scope in self.scope_context.call.iter().rev() {
      if scope.callee.node == node {
        created_in_self = true;
        break;
      }
    }

    if created_in_self {
      function.consume_body(self);
      self.factory.unknown
    } else {
      function
    }
  }
}
