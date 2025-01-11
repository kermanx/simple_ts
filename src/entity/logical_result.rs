use super::{
  Entity, EntityFactory, EntityTrait, EnumeratedProperties, IteratedElements, TypeofResult,
};
use crate::analyzer::Analyzer;
use oxc::ast::ast::LogicalOperator;

#[derive(Debug, Clone)]
pub struct LogicalResultEntity<'a> {
  pub value: Entity<'a>,
  pub is_coalesce: bool,
  pub result: Option<bool>,
}

impl<'a> EntityTrait<'a> for LogicalResultEntity<'a> {
  fn unknown_mutation(&'a self, analyzer: &mut Analyzer<'a>) {
    self.value.unknown_mutation(analyzer);
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    self.value.get_property(analyzer, key)
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    self.value.set_property(analyzer, key, value);
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    self.value.enumerate_properties(analyzer)
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    self.value.delete_property(analyzer, key);
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    self.value.call(analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    self.value.construct(analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, props: Entity<'a>) -> Entity<'a> {
    self.value.jsx(analyzer, props)
  }

  fn r#await(&'a self, analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    self.value.r#await(analyzer)
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    self.value.iterate(analyzer)
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.value.get_typeof(analyzer)
  }

  fn get_to_string(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.value.get_to_string(analyzer)
  }

  fn get_to_numeric(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.value.get_to_numeric(analyzer)
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    let value = self.value.get_to_boolean(analyzer);
    if self.is_coalesce {
      value
    } else if let Some(result) = self.result {
      analyzer.factory.boolean_literal(result)
    } else {
      value
    }
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.value.get_to_property_key(analyzer)
  }

  fn get_to_jsx_child(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.value.get_to_jsx_child(analyzer)
  }

  fn test_typeof(&self) -> TypeofResult {
    self.value.test_typeof()
  }

  fn test_truthy(&self) -> Option<bool> {
    if self.is_coalesce {
      self.value.test_truthy()
    } else {
      self.result
    }
  }

  fn test_nullish(&self) -> Option<bool> {
    if self.is_coalesce {
      self.result
    } else {
      self.value.test_nullish()
    }
  }
}

impl<'a> EntityFactory<'a> {
  /// Only used when (maybe_left, maybe_right) == (true, true)
  pub fn logical_result(
    &self,
    left: Entity<'a>,
    right: Entity<'a>,
    operator: LogicalOperator,
  ) -> &'a mut LogicalResultEntity<'a> {
    self.alloc(LogicalResultEntity {
      value: self.union((left, right)),
      is_coalesce: operator == LogicalOperator::Coalesce,
      result: match operator {
        LogicalOperator::Or => match right.test_truthy() {
          Some(true) => Some(true),
          _ => None,
        },
        LogicalOperator::And => match right.test_truthy() {
          Some(false) => Some(false),
          _ => None,
        },
        LogicalOperator::Coalesce => match right.test_nullish() {
          Some(false) => Some(false),
          _ => None,
        },
      },
    })
  }
}
