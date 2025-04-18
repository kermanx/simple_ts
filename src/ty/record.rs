use std::{fmt::Debug, hash::Hash};

use oxc::{
  allocator::{self},
  ast::ast::{PropertyKey, TSSignature, TSType},
  semantic::SymbolId,
  span::SPAN,
};
use oxc_syntax::number::ToJsString;

use super::{Ty, accumulator::TypeAccumulator, property_key::PropertyKeyType};
use crate::{allocator::Allocator, analyzer::Analyzer, ty::union::UnionType};

#[derive(Debug, Clone, Copy)]
pub struct RecordPropertyValue<'a> {
  pub value: Ty<'a>,
  pub optional: bool,
  pub readonly: bool,
}

pub struct KeyedPropertyMap<'a, K>(pub allocator::HashMap<'a, K, RecordPropertyValue<'a>>);

impl<K: Debug> Debug for KeyedPropertyMap<'_, K> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_map().entries(self.0.iter()).finish()
  }
}

impl<'a, K: Eq + Hash> KeyedPropertyMap<'a, K> {
  pub fn init(&mut self, analyzer: &mut Analyzer<'a>, key: K, mut value: RecordPropertyValue<'a>) {
    fn union_is_bivariant(u: &UnionType<'_>) -> bool {
      let mut bivariant = true;
      u.for_each(|ty| match ty {
        Ty::Function(f) => bivariant &= f.is_method,
        _ => bivariant = false,
      });
      bivariant
    }

    match self.0.entry(key) {
      allocator::hash_map::Entry::Occupied(mut entry) => {
        let prev = entry.get();
        value.value = match (prev.value, value.value) {
          (Ty::Function(f1), Ty::Function(f2)) if f1.is_method && f2.is_method => {
            analyzer.into_union([prev.value, value.value]).unwrap()
          }
          (Ty::Union(u1), Ty::Function(f2)) if union_is_bivariant(u1) && f2.is_method => {
            analyzer.into_union([prev.value, value.value]).unwrap()
          }
          _ => value.value,
        };
        entry.insert(value);
      }
      allocator::hash_map::Entry::Vacant(entry) => {
        entry.insert(value);
      }
    }
  }

  pub fn get(&self, key: K) -> Ty<'a> {
    if let Some(property) = self.0.get(&key) { property.value } else { Ty::Error }
  }
}

#[derive(Debug, Default)]
pub struct MappedPropertyBuilder<'a> {
  value: TypeAccumulator<'a>,
  readonly: bool,
}

pub struct RecordTypeBuilder<'a> {
  pub string_keyed: KeyedPropertyMap<'a, &'a str>,
  pub symbol_keyed: KeyedPropertyMap<'a, SymbolId>,

  pub string_mapped: MappedPropertyBuilder<'a>,
  pub number_mapped: MappedPropertyBuilder<'a>,
  pub symbol_mapped: MappedPropertyBuilder<'a>,
}

impl<'a> RecordTypeBuilder<'a> {
  pub fn new_in(allocator: Allocator<'a>) -> Self {
    Self {
      string_keyed: KeyedPropertyMap(allocator::HashMap::new_in(&allocator)),
      symbol_keyed: KeyedPropertyMap(allocator::HashMap::new_in(&allocator)),
      string_mapped: MappedPropertyBuilder::default(),
      number_mapped: MappedPropertyBuilder::default(),
      symbol_mapped: MappedPropertyBuilder::default(),
    }
  }

  pub fn init_property(
    &mut self,
    analyzer: &mut Analyzer<'a>,
    key: PropertyKeyType<'a>,
    value: Ty<'a>,
    optional: bool,
    readonly: bool,
  ) {
    let keyed_property = RecordPropertyValue { value, optional, readonly };
    match key {
      PropertyKeyType::Error => {}
      PropertyKeyType::AnyString => {
        self.string_mapped.value.add(value, analyzer.allocator);
      }
      PropertyKeyType::AnyNumber => {
        self.string_mapped.value.add(value, analyzer.allocator);
        self.number_mapped.value.add(value, analyzer.allocator);
      }
      PropertyKeyType::AnySymbol => {
        self.symbol_mapped.value.add(value, analyzer.allocator);
      }
      PropertyKeyType::StringLiteral(s) => {
        self.string_keyed.init(analyzer, s.as_str(), keyed_property);
      }
      PropertyKeyType::NumericLiteral(n) => {
        let s = analyzer.allocator.alloc_str(&n.0.to_js_string());
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

  pub fn remove_property(&mut self, analyzer: &mut Analyzer<'a>, key: PropertyKeyType<'a>) {
    todo!()
  }

  pub fn build(mut self) -> RecordType<'a> {
    RecordType {
      string_keyed: self.string_keyed,
      symbol_keyed: self.symbol_keyed,
      string_mapped: self.string_mapped.value.to_ty().map(|ty| RecordPropertyValue {
        value: ty,
        optional: false,
        readonly: self.string_mapped.readonly,
      }),
      number_mapped: self.number_mapped.value.to_ty().map(|ty| RecordPropertyValue {
        value: ty,
        optional: false,
        readonly: self.number_mapped.readonly,
      }),
      symbol_mapped: self.symbol_mapped.value.to_ty().map(|ty| RecordPropertyValue {
        value: ty,
        optional: false,
        readonly: self.symbol_mapped.readonly,
      }),
    }
  }
}

#[derive(Debug)]
pub struct RecordType<'a> {
  pub string_keyed: KeyedPropertyMap<'a, &'a str>,
  pub symbol_keyed: KeyedPropertyMap<'a, SymbolId>,

  pub string_mapped: Option<RecordPropertyValue<'a>>,
  pub number_mapped: Option<RecordPropertyValue<'a>>,
  pub symbol_mapped: Option<RecordPropertyValue<'a>>,
}

impl<'a> RecordType<'a> {
  pub fn new_in(allocator: Allocator<'a>) -> Self {
    Self {
      string_keyed: KeyedPropertyMap(allocator::HashMap::new_in(&allocator)),
      symbol_keyed: KeyedPropertyMap(allocator::HashMap::new_in(&allocator)),
      string_mapped: None,
      number_mapped: None,
      symbol_mapped: None,
    }
  }

  pub fn get_property(&self, key: PropertyKeyType<'a>) -> Ty<'a> {
    match key {
      PropertyKeyType::Error => Ty::Error,
      PropertyKeyType::AnyString => self.string_mapped.as_ref().map_or(Ty::Error, |p| p.value),
      PropertyKeyType::AnyNumber => self.number_mapped.as_ref().map_or(Ty::Error, |p| p.value),
      PropertyKeyType::AnySymbol => self.symbol_mapped.as_ref().map_or(Ty::Error, |p| p.value),
      PropertyKeyType::StringLiteral(s) => self.string_keyed.get(s.as_str()),
      PropertyKeyType::NumericLiteral(n) => self.string_keyed.get(n.0.to_js_string().as_str()),
      PropertyKeyType::UniqueSymbol(s) => self.symbol_keyed.get(s),
    }
  }

  pub fn extend(&mut self, other: &RecordType<'a>) {
    // FIXME: overload
    self.string_keyed.0.extend(&other.string_keyed.0);
    self.symbol_keyed.0.extend(&other.symbol_keyed.0);
    self.string_mapped = other.string_mapped;
    self.number_mapped = other.number_mapped;
    self.symbol_mapped = other.symbol_mapped;
  }

  pub fn is_empty(&self) -> bool {
    self.string_keyed.0.is_empty()
      && self.symbol_keyed.0.is_empty()
      && self.string_mapped.is_none()
      && self.number_mapped.is_none()
      && self.symbol_mapped.is_none()
  }
}

impl<'a> Analyzer<'a> {
  fn serialize_keyed_property(
    &mut self,
    key: PropertyKey<'a>,
    property: &RecordPropertyValue<'a>,
  ) -> TSSignature<'a> {
    self.ast_builder.ts_signature_property_signature(
      SPAN,
      false,
      property.optional,
      property.readonly,
      key,
      Some(self.ast_builder.ts_type_annotation(SPAN, self.serialize_type(property.value))),
    )
  }

  fn serialize_mapped_property(
    &mut self,
    key_type: TSType<'a>,
    property: &Option<RecordPropertyValue<'a>>,
  ) -> Option<TSSignature<'a>> {
    let property = property.as_ref()?;
    Some(self.ast_builder.ts_signature_index_signature(
      SPAN,
      self.ast_builder.vec1(self.ast_builder.ts_index_signature_name(
        SPAN,
        "1",
        self.ast_builder.ts_type_annotation(SPAN, key_type),
      )),
      self.ast_builder.ts_type_annotation(SPAN, self.serialize_type(property.value)),
      property.readonly,
      false,
    ))
  }

  pub fn serialize_record_type(&mut self, record: &RecordType<'a>) -> TSType<'a> {
    let mut members = self.ast_builder.vec();
    for (key, property) in &record.string_keyed.0 {
      members.push(self.serialize_keyed_property(
        self.ast_builder.property_key_static_identifier(SPAN, *key),
        property,
      ));
    }
    for (key, property) in &record.symbol_keyed.0 {
      members.push(self.serialize_keyed_property(todo!(), property));
    }
    if let Some(node) = self.serialize_mapped_property(
      self.ast_builder.ts_type_number_keyword(SPAN),
      &record.number_mapped,
    ) {
      members.push(node);
    }
    if let Some(node) = self.serialize_mapped_property(
      self.ast_builder.ts_type_symbol_keyword(SPAN),
      &record.symbol_mapped,
    ) {
      members.push(node);
    }
    if let Some(node) = self.serialize_mapped_property(
      self.ast_builder.ts_type_string_keyword(SPAN),
      &record.string_mapped,
    ) {
      members.push(node);
    }
    self.ast_builder.ts_type_type_literal(SPAN, members)
  }
}
