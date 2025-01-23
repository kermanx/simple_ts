use std::{cell::RefCell, collections::hash_map::Entry, hash::Hash};

use oxc::{
  ast::ast::{PropertyKey, TSSignature, TSType},
  semantic::SymbolId,
  span::SPAN,
};
use oxc_syntax::number::ToJsString;
use rustc_hash::FxHashMap;

use super::{accumulator::TypeAccumulator, property_key::PropertyKeyType, Ty};
use crate::{analyzer::Analyzer, ty::union::UnionType};

#[derive(Debug, Clone)]
pub struct KeyedProperty<'a> {
  pub value: Ty<'a>,
  pub optional: bool,
  pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct KeyedPropertyMap<'a, K>(pub FxHashMap<K, KeyedProperty<'a>>);

// FIXME: Why is this not derived?
impl<'a, K> Default for KeyedPropertyMap<'a, K> {
  fn default() -> Self {
    Self(FxHashMap::default())
  }
}

impl<'a, K: Eq + Hash> KeyedPropertyMap<'a, K> {
  pub fn init(&mut self, analyzer: &mut Analyzer<'a>, key: K, mut value: KeyedProperty<'a>) {
    fn union_is_bivariant<'a>(u: &UnionType<'a>) -> bool {
      let mut bivariant = true;
      u.for_each(|ty| match ty {
        Ty::Function(f) => bivariant &= f.bivariant,
        _ => bivariant = false,
      });
      bivariant
    }

    match self.0.entry(key) {
      Entry::Occupied(mut entry) => {
        let prev = entry.get();
        value.value = match (prev.value, value.value) {
          (Ty::Function(f1), Ty::Function(f2)) if f1.bivariant && f2.bivariant => {
            analyzer.into_union([prev.value, value.value]).unwrap()
          }
          (Ty::Union(u1), Ty::Function(f2)) if union_is_bivariant(u1) && f2.bivariant => {
            analyzer.into_union([prev.value, value.value]).unwrap()
          }
          _ => value.value,
        };
        entry.insert(value);
      }
      Entry::Vacant(entry) => {
        entry.insert(value);
      }
    }
  }

  pub fn get(&self, key: K) -> Ty<'a> {
    if let Some(property) = self.0.get(&key) {
      property.value.clone()
    } else {
      Ty::Error
    }
  }
}

#[derive(Debug, Default)]
pub struct MappedProperty<'a> {
  value: RefCell<TypeAccumulator<'a>>,
  readonly: bool,
}

impl<'a> Clone for MappedProperty<'a> {
  fn clone(&self) -> Self {
    Self { value: RefCell::new(self.value.borrow_mut().duplicate()), readonly: self.readonly }
  }
}

#[derive(Debug, Default)]
pub struct RecordType<'a> {
  pub string_keyed: KeyedPropertyMap<'a, &'a str>,
  pub symbol_keyed: KeyedPropertyMap<'a, SymbolId>,

  pub string_mapped: MappedProperty<'a>,
  pub number_mapped: MappedProperty<'a>,
  pub symbol_mapped: MappedProperty<'a>,
}

impl<'a> RecordType<'a> {
  pub fn init_property(
    &mut self,
    analyzer: &mut Analyzer<'a>,
    key: PropertyKeyType<'a>,
    value: Ty<'a>,
    optional: bool,
    readonly: bool,
  ) {
    let keyed_property = KeyedProperty { value, optional, readonly };
    match key {
      PropertyKeyType::Error => {}
      PropertyKeyType::AnyString => {
        self.string_mapped.value.borrow_mut().add(value, analyzer.allocator);
      }
      PropertyKeyType::AnyNumber => {
        self.string_mapped.value.borrow_mut().add(value, analyzer.allocator);
        self.number_mapped.value.borrow_mut().add(value, analyzer.allocator);
      }
      PropertyKeyType::AnySymbol => {
        self.symbol_mapped.value.borrow_mut().add(value, analyzer.allocator);
      }
      PropertyKeyType::StringLiteral(s) => {
        self.string_keyed.init(analyzer, s.as_str(), keyed_property);
      }
      PropertyKeyType::NumericLiteral(n) => {
        let s = analyzer.allocator.alloc(n.0.to_js_string());
        self.string_keyed.init(analyzer, s, keyed_property);
      }
      PropertyKeyType::UniqueSymbol(s) => {
        self.symbol_keyed.init(analyzer, s, keyed_property);
      }
    }
  }

  pub fn init_spread(&mut self, analyzer: &mut Analyzer<'a>, value: Ty<'a>) {
    todo!()
  }

  pub fn get_property(&self, key: PropertyKeyType<'a>) -> Ty<'a> {
    match key {
      PropertyKeyType::Error => Ty::Error,
      PropertyKeyType::AnyString => {
        self.string_mapped.value.borrow_mut().to_ty().unwrap_or(Ty::Error)
      }
      PropertyKeyType::AnyNumber => {
        self.number_mapped.value.borrow_mut().to_ty().unwrap_or(Ty::Error)
      }
      PropertyKeyType::AnySymbol => {
        self.symbol_mapped.value.borrow_mut().to_ty().unwrap_or(Ty::Error)
      }
      PropertyKeyType::StringLiteral(s) => self.string_keyed.get(s.as_str()),
      PropertyKeyType::NumericLiteral(n) => self.string_keyed.get(n.0.to_js_string().as_str()),
      PropertyKeyType::UniqueSymbol(s) => self.symbol_keyed.get(s),
    }
  }

  pub fn delete_property(&mut self, analyzer: &mut Analyzer<'a>, key: PropertyKeyType<'a>) {
    todo!()
  }

  pub fn extend(&mut self, other: &RecordType<'a>) {
    // FIXME: overload
    self.string_keyed.0.extend(other.string_keyed.0.clone());
    self.symbol_keyed.0.extend(other.symbol_keyed.0.clone());
    self.string_mapped = other.string_mapped.clone();
    self.number_mapped = other.number_mapped.clone();
    self.symbol_mapped = other.symbol_mapped.clone();
  }

  pub fn is_empty(&self) -> bool {
    self.string_keyed.0.is_empty()
      && self.symbol_keyed.0.is_empty()
      && self.string_mapped.value.borrow().is_empty()
      && self.number_mapped.value.borrow().is_empty()
      && self.symbol_mapped.value.borrow().is_empty()
  }
}

impl<'a> Analyzer<'a> {
  fn print_keyed_property(
    &mut self,
    key: PropertyKey<'a>,
    property: &KeyedProperty<'a>,
  ) -> TSSignature<'a> {
    self.ast_builder.ts_signature_property_signature(
      SPAN,
      false,
      property.optional,
      property.readonly,
      key,
      Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(property.value))),
    )
  }

  fn print_mapped_property(
    &mut self,
    key_type: TSType<'a>,
    property: &MappedProperty<'a>,
  ) -> Option<TSSignature<'a>> {
    let ty = property.value.borrow_mut().to_ty()?;
    Some(self.ast_builder.ts_signature_index_signature(
      SPAN,
      self.ast_builder.vec1(self.ast_builder.ts_index_signature_name(
        SPAN,
        "1",
        self.ast_builder.ts_type_annotation(SPAN, key_type),
      )),
      self.ast_builder.ts_type_annotation(SPAN, self.print_type(ty)),
      property.readonly,
      false,
    ))
  }

  pub fn print_record_type(&mut self, record: &RecordType<'a>) -> TSType<'a> {
    let mut members = self.ast_builder.vec();
    for (key, property) in &record.string_keyed.0 {
      members.push(
        self.print_keyed_property(
          self.ast_builder.property_key_identifier_name(SPAN, *key),
          property,
        ),
      );
    }
    for (key, property) in &record.symbol_keyed.0 {
      members.push(self.print_keyed_property(todo!(), property));
    }
    if let Some(node) = self
      .print_mapped_property(self.ast_builder.ts_type_number_keyword(SPAN), &record.number_mapped)
    {
      members.push(node);
    }
    if let Some(node) = self
      .print_mapped_property(self.ast_builder.ts_type_symbol_keyword(SPAN), &record.symbol_mapped)
    {
      members.push(node);
    }
    if let Some(node) = self
      .print_mapped_property(self.ast_builder.ts_type_string_keyword(SPAN), &record.string_mapped)
    {
      members.push(node);
    }
    self.ast_builder.ts_type_type_literal(SPAN, members)
  }
}
