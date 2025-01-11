use super::{
  consumed_object, Entity, EntityFactory, EntityTrait, EnumeratedProperties, IteratedElements,
  LiteralEntity, TypeofResult,
};
use crate::{analyzer::Analyzer, use_consumed_flag};
use oxc::semantic::{ScopeId, SymbolId};
use std::{
  cell::{Cell, RefCell},
  fmt,
};

pub struct ArrayEntity<'a> {
  consumed: Cell<bool>,
  cf_scope: ScopeId,
  object_id: SymbolId,
  pub elements: RefCell<Vec<Entity<'a>>>,
  pub rest: RefCell<Vec<Entity<'a>>>,
}

impl<'a> fmt::Debug for ArrayEntity<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ArrayEntity")
      .field("consumed", &self.consumed.get())
      .field("elements", &self.elements.borrow())
      .field("rest", &self.rest.borrow())
      .finish()
  }
}

impl<'a> EntityTrait<'a> for ArrayEntity<'a> {
  fn unknown_mutation(&'a self, analyzer: &mut Analyzer<'a>) {
    use_consumed_flag!(self);

    analyzer.mark_object_consumed(self.cf_scope, self.object_id);

    for element in self.elements.borrow().iter() {
      element.unknown_mutation(analyzer);
    }
    for rest in self.rest.borrow().iter() {
      rest.unknown_mutation(analyzer);
    }
  }

  fn get_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::get_property(self, analyzer, key);
    }

    analyzer.mark_object_property_exhaustive_read(self.cf_scope, self.object_id);

    let key = key.get_to_property_key(analyzer);
    if let Some(key_literals) = key.get_to_literals(analyzer) {
      let mut result = vec![];
      let mut rest_added = false;
      for key_literal in key_literals {
        match key_literal {
          LiteralEntity::String(key) => {
            if let Ok(index) = key.parse::<usize>() {
              if let Some(element) = self.elements.borrow().get(index) {
                result.push(*element);
              } else if !rest_added {
                rest_added = true;
                result.extend(self.rest.borrow().iter().copied());
                result.push(analyzer.factory.undefined);
              }
            } else if key == "length" {
              result.push(self.get_length().map_or_else(
                || analyzer.factory.unknown_number,
                |length| analyzer.factory.number(length as f64, None),
              ));
            } else if let Some(property) = analyzer.builtins.prototypes.array.get_string_keyed(key)
            {
              result.push(property);
            } else {
              result.push(analyzer.factory.unmatched_prototype_property);
            }
          }
          LiteralEntity::Symbol(key, _) => todo!(),
          _ => unreachable!("Invalid property key"),
        }
      }
      analyzer.factory.union(result)
    } else {
      analyzer.factory.unknown
    }
  }

  fn set_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>, value: Entity<'a>) {
    if self.consumed.get() {
      return consumed_object::set_property(analyzer, key, value);
    }

    let (has_exhaustive, indeterminate) = analyzer.pre_mutate_object(self.cf_scope, self.object_id);

    if has_exhaustive {
      self.unknown_mutation(analyzer);
      return consumed_object::set_property(analyzer, key, value);
    }

    'known: {
      let Some(key_literals) = key.get_to_property_key(analyzer).get_to_literals(analyzer) else {
        break 'known;
      };

      let definite = !indeterminate && key_literals.len() == 1;
      let mut rest_added = false;
      for key_literal in key_literals {
        match key_literal {
          LiteralEntity::String(key_str) => {
            if let Ok(index) = key_str.parse::<usize>() {
              if let Some(element) = self.elements.borrow_mut().get_mut(index) {
                *element = if definite { value } else { analyzer.factory.union((*element, value)) };
              } else if !rest_added {
                rest_added = true;
                self.rest.borrow_mut().push(value);
              }
            } else if key_str == "length" {
              if let Some(length) = value.get_literal(analyzer).and_then(|lit| lit.to_number()) {
                if let Some(length) = length.map(|l| l.0.trunc()) {
                  let length = length as usize;
                  let mut elements = self.elements.borrow_mut();
                  let mut rest = self.rest.borrow_mut();
                  if elements.len() > length {
                    elements.truncate(length);
                    rest.clear();
                  } else if !rest.is_empty() {
                    rest.push(analyzer.factory.undefined);
                  } else if elements.len() < length {
                    for _ in elements.len()..length {
                      elements.push(analyzer.factory.undefined);
                    }
                  }
                } else {
                  analyzer.thrown_builtin_error("Invalid array length");
                }
              }
            } else {
              break 'known;
            }
          }
          LiteralEntity::Symbol(key, _) => todo!(),
          _ => unreachable!("Invalid property key"),
        }
      }
      return;
    }
  }

  fn enumerate_properties(&'a self, analyzer: &mut Analyzer<'a>) -> EnumeratedProperties<'a> {
    if self.consumed.get() {
      return consumed_object::enumerate_properties(self, analyzer);
    }

    analyzer.mark_object_property_exhaustive_read(self.cf_scope, self.object_id);

    let mut entries = Vec::new();
    for (i, element) in self.elements.borrow().iter().enumerate() {
      entries.push((
        true,
        analyzer.factory.string(analyzer.allocator.alloc(i.to_string())),
        *element,
      ));
    }
    let rest = self.rest.borrow();
    if !rest.is_empty() {
      entries.push((
        true,
        analyzer.factory.unknown_string,
        analyzer.factory.union(rest.iter().cloned().collect::<Vec<_>>()),
      ));
    }

    entries
  }

  fn delete_property(&'a self, analyzer: &mut Analyzer<'a>, key: Entity<'a>) {
    if self.consumed.get() {
      return consumed_object::delete_property(analyzer, key);
    }

    let (has_exhaustive, _) = analyzer.pre_mutate_object(self.cf_scope, self.object_id);

    if has_exhaustive {
      self.unknown_mutation(analyzer);
      return consumed_object::delete_property(analyzer, key);
    }
  }

  fn call(&'a self, analyzer: &mut Analyzer<'a>, this: Entity<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::call(self, analyzer, this, args)
  }

  fn construct(&'a self, analyzer: &mut Analyzer<'a>, args: Entity<'a>) -> Entity<'a> {
    consumed_object::construct(self, analyzer, args)
  }

  fn jsx(&'a self, analyzer: &mut Analyzer<'a>, props: Entity<'a>) -> Entity<'a> {
    consumed_object::jsx(self, analyzer, props)
  }

  fn r#await(&'a self, analyzer: &mut Analyzer<'a>) -> Entity<'a> {
    if self.consumed.get() {
      return consumed_object::r#await(analyzer);
    }
    self
  }

  fn iterate(&'a self, analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
    if self.consumed.get() {
      return consumed_object::iterate(analyzer);
    }

    analyzer.mark_object_property_exhaustive_read(self.cf_scope, self.object_id);

    (self.elements.borrow().clone(), analyzer.factory.try_union(self.rest.borrow().clone()))
  }

  fn get_typeof(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.string("object")
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
    analyzer.factory.unknown
  }

  fn get_to_boolean(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    analyzer.factory.boolean(true)
  }

  fn get_to_property_key(&'a self, analyzer: &Analyzer<'a>) -> Entity<'a> {
    self.get_to_string(analyzer)
  }

  fn get_to_jsx_child(&'a self, _analyzer: &Analyzer<'a>) -> Entity<'a> {
    self
  }

  fn test_typeof(&self) -> TypeofResult {
    TypeofResult::Object
  }

  fn test_truthy(&self) -> Option<bool> {
    Some(true)
  }

  fn test_nullish(&self) -> Option<bool> {
    Some(false)
  }
}

impl<'a> ArrayEntity<'a> {
  pub fn push_element(&self, element: Entity<'a>) {
    if self.rest.borrow().is_empty() {
      self.elements.borrow_mut().push(element);
    } else {
      self.init_rest(element);
    }
  }

  pub fn init_rest(&self, rest: Entity<'a>) {
    self.rest.borrow_mut().push(rest);
  }

  pub fn get_length(&self) -> Option<usize> {
    if self.rest.borrow().is_empty() {
      Some(self.elements.borrow().len())
    } else {
      None
    }
  }
}

impl<'a> EntityFactory<'a> {
  pub fn array(&self, cf_scope: ScopeId, object_id: SymbolId) -> &'a mut ArrayEntity<'a> {
    self.alloc(ArrayEntity {
      consumed: Cell::new(false),
      cf_scope,
      object_id,
      elements: RefCell::new(Vec::new()),
      rest: RefCell::new(Vec::new()),
    })
  }
}

impl<'a> Analyzer<'a> {
  pub fn new_empty_array(&mut self) -> &'a mut ArrayEntity<'a> {
    self.factory.array(self.scope_context.cf.current_id(), self.scope_context.alloc_object_id())
  }
}
