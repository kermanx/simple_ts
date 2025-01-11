use super::{
  consumed_object, Entity, EntityFactory, EntityTrait, EnumeratedProperties, IteratedElements,
  ObjectEntity, TypeofResult,
};
use crate::analyzer::Analyzer;
use std::fmt::Debug;

pub trait BuiltinFnEntity<'a>: Debug {
  #[cfg(feature = "flame")]
  fn name(&self) -> &'static str;
  fn object(&self) -> Option<&'a ObjectEntity<'a>> {
    None
  }
  fn call_impl(
    &self,
    analyzer: &mut Analyzer<'a>,

    this: Entity<'a>,
    args: Entity<'a>,
  ) -> Entity<'a>;
}

impl<'a, T: BuiltinFnEntity<'a>> EntityTrait<'a> for T {
  fn unknown_mutation(&'a self, _analyzer: &mut Analyzer<'a>) {
    // No effect
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    if let Some(object) = self.object() {
      object.get_property(analyzer, key)
    } else {
      analyzer.builtins.prototypes.function.get_property(analyzer, key)
    }
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    if let Some(object) = self.object() {
      object.set_property(analyzer, key, value)
    } else {
      analyzer.add_diagnostic(
      "Should not set property of builtin function, it may cause unexpected tree-shaking behavior",
    );
      consumed_object::set_property(analyzer, key, value)
    }
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    if let Some(object) = self.object() {
      object.delete_property(analyzer, key)
    } else {
      analyzer.add_diagnostic("Should not delete property of builtin function, it may cause unexpected tree-shaking behavior");
      consumed_object::delete_property(analyzer, key)
    }
  }

  fn enumerate_properties(&'a self, _analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    vec![]
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    #[cfg(feature = "flame")]
    let _scope_guard = flame::start_guard(self.name());
    self.call_impl(analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, props: Entity<'a>) -> Entity<'a> {
    self.call_impl(
      analyzer,
      analyzer.factory.unknown,
      analyzer.factory.arguments(vec![(false, props)]),
    )
  }

  fn r#await(&'a self, _analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    self
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    analyzer.thrown_builtin_error("Cannot iterate over function");
    consumed_object::iterate(analyzer)
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string("function")
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.unknown_string
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.nan
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.boolean(true)
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.get_to_string(analyzer)
  }

  fn get_to_jsx_child(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    // TODO: analyzer.thrown_builtin_error("Functions are not valid JSX children");
    analyzer.factory.string("")
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

pub trait BuiltinFnImplementation<'a>:
  Fn(&mut Analyzer<'a>, Entity<'a>, Entity<'a>) -> Entity<'a>
{
}
impl<'a, T: Fn(&mut Analyzer<'a>, Entity<'a>, Entity<'a>) -> Entity<'a>> BuiltinFnImplementation<'a>
  for T
{
}

#[derive(Clone, Copy)]
pub struct ImplementedBuiltinFnEntity<'a, F: BuiltinFnImplementation<'a> + 'a> {
  #[cfg(feature = "flame")]
  name: &'static str,
  implementation: F,
  object: Option<&'a ObjectEntity<'a>>,
}

impl<'a, F: BuiltinFnImplementation<'a> + 'a> Debug for ImplementedBuiltinFnEntity<'a, F> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ImplementedBuiltinFnEntity").finish()
  }
}

impl<'a, F: BuiltinFnImplementation<'a> + 'a> BuiltinFnEntity<'a>
  for ImplementedBuiltinFnEntity<'a, F>
{
  #[cfg(feature = "flame")]
  fn name(&self) -> &'static str {
    self.name
  }
  fn object(&self) -> Option<&'a ObjectEntity<'a>> {
    self.object
  }
  fn call_impl(
    &self,
    analyzer: &mut Analyzer<'a>,

    this: Entity<'a>,
    args: Entity<'a>,
  ) -> Entity<'a> {
    (self.implementation)(analyzer, this, args)
  }
}

impl<'a> EntityFactory<'a> {
  pub fn implemented_builtin_fn<F: BuiltinFnImplementation<'a> + 'a>(
    &self,
    name: &'static str,
    implementation: F,
  ) -> Entity<'a> {
    self.alloc(ImplementedBuiltinFnEntity {
      #[cfg(feature = "flame")]
      name,
      implementation,
      object: None,
    })
  }
}

impl<'a> Analyzer<'a> {
  pub fn dynamic_implemented_builtin<F: BuiltinFnImplementation<'a> + 'a>(
    &mut self,
    name: &'static str,
    implementation: F,
  ) -> Entity<'a> {
    self.factory.alloc(ImplementedBuiltinFnEntity {
      #[cfg(feature = "flame")]
      name,
      implementation,
      object: Some(self.new_function_object()),
    })
  }
}

#[derive(Debug, Clone)]
pub struct PureBuiltinFnEntity<'a> {
  return_value: fn(&EntityFactory<'a>) -> Entity<'a>,
}

impl<'a> BuiltinFnEntity<'a> for PureBuiltinFnEntity<'a> {
  #[cfg(feature = "flame")]
  fn name(&self) -> &'static str {
    "<PureBuiltin>"
  }

  fn call_impl(
    &self,
    analyzer: &mut Analyzer<'a>,

    this: Entity<'a>,
    args: Entity<'a>,
  ) -> Entity<'a> {
    this.unknown_mutation(analyzer);
    args.unknown_mutation(analyzer);
    (self.return_value)(analyzer.factory)
  }
}

impl<'a> PureBuiltinFnEntity<'a> {
  pub fn new(return_value: fn(&EntityFactory<'a>) -> Entity<'a>) -> Self {
    Self { return_value }
  }
}
