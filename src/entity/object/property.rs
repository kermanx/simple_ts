use crate::{analyzer::Analyzer, entity::Entity};

#[derive(Debug, Clone, Copy)]
pub enum ObjectPropertyValue<'a> {
  /// (value, readonly)
  Field(Entity<'a>, bool),
  /// (getter, setter)
  Property(Option<Entity<'a>>, Option<Entity<'a>>),
}

#[derive(Debug)]
pub struct ObjectProperty<'a> {
  pub definite: bool,                                // 是否一定存在
  pub possible_values: Vec<ObjectPropertyValue<'a>>, // 可能的值，可能有多个
}

impl<'a> Default for ObjectProperty<'a> {
  fn default() -> Self {
    Self { definite: true, possible_values: vec![] }
  }
}

impl<'a> ObjectProperty<'a> {
  pub fn get(
    &mut self,
    analyzer: &Analyzer<'a>,
    values: &mut Vec<Entity<'a>>,
    getters: &mut Vec<Entity<'a>>,
    definite: &mut bool,
  ) {
    for possible_value in &self.possible_values {
      match possible_value {
        ObjectPropertyValue::Field(value, _) => values.push(*value),
        ObjectPropertyValue::Property(Some(getter), _) => getters.push(*getter),
        ObjectPropertyValue::Property(None, _) => values.push(analyzer.factory.undefined),
      }
    }

    *definite &= self.definite;
  }

  pub fn set(
    &mut self,
    _analyzer: &Analyzer<'a>,
    indeterminate: bool,
    value: Entity<'a>,
    setters: &mut Vec<(bool, Entity<'a>)>,
  ) {
    let mut writable = false;
    let call_setter_indeterminately = indeterminate || self.possible_values.len() > 1;
    for possible_value in &self.possible_values {
      match *possible_value {
        ObjectPropertyValue::Field(_, false) => writable = true,
        ObjectPropertyValue::Property(_, Some(setter)) => {
          setters.push((call_setter_indeterminately, setter))
        }
        _ => {}
      }
    }

    if !indeterminate {
      // Remove all writable fields
      self.possible_values = self
        .possible_values
        .iter()
        .filter(|possible_value| !matches!(possible_value, ObjectPropertyValue::Field(_, false)))
        .cloned()
        .collect();
    }

    if writable {
      self.possible_values.push(ObjectPropertyValue::Field(value, false));
    }
  }

  pub fn delete(&mut self, indeterminate: bool) {
    self.definite = false;
    if !indeterminate {
      self.possible_values.clear();
    }
  }

  pub fn consume(self, analyzer: &mut Analyzer<'a>) {
    for possible_value in self.possible_values {
      match possible_value {
        ObjectPropertyValue::Field(value, _) => value.unknown_mutation(analyzer),
        ObjectPropertyValue::Property(getter, setter) => {
          getter.map(|v| v.unknown_mutation(analyzer));
          setter.map(|v| v.unknown_mutation(analyzer));
        }
      }
    }
  }
}
