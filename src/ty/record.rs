use super::{accumulator::TypeAccumulator, property_key::PropertyKeyType, Ty};
use crate::analyzer::Analyzer;
use oxc::{
  ast::ast::{PropertyKey, TSSignature, TSType},
  semantic::SymbolId,
  span::SPAN,
};
use oxc_syntax::number::ToJsString;
use rustc_hash::FxHashMap;
use std::cell::{Cell, RefCell};

#[derive(Debug, Clone)]
pub struct KeyedProperty<'a> {
  value: Ty<'a>,
  optional: bool,
  readonly: bool,
}

#[derive(Debug, Default)]
pub struct MappedProperty<'a> {
  value: RefCell<TypeAccumulator<'a>>,
  readonly: bool,
}

#[derive(Debug, Default)]
pub struct RecordType<'a> {
  pub proto: Option<Ty<'a>>,

  pub string_keyed: FxHashMap<&'a str, KeyedProperty<'a>>,
  pub symbol_keyed: FxHashMap<SymbolId, KeyedProperty<'a>>,

  pub string_mapped: MappedProperty<'a>,
  pub number_mapped: MappedProperty<'a>,
  pub symbol_mapped: MappedProperty<'a>,
}

impl<'a> RecordType<'a> {
  pub fn init_proto(&mut self, proto: Ty<'a>) {
    self.proto = Some(proto);
  }

  pub fn init_property(
    &mut self,
    analyzer: &mut Analyzer<'a>,
    key: Ty<'a>,
    value: Ty<'a>,
    optional: bool,
    readonly: bool,
  ) {
    let keyed_property = KeyedProperty { value, optional, readonly };
    match analyzer.to_property_key(key) {
      PropertyKeyType::Error => {}
      PropertyKeyType::AnyString => {
        self.string_mapped.value.borrow_mut().add(value, analyzer.allocator);
      }
      PropertyKeyType::AnyNumber => {
        self.number_mapped.value.borrow_mut().add(value, analyzer.allocator);
      }
      PropertyKeyType::AnySymbol => {
        self.symbol_mapped.value.borrow_mut().add(value, analyzer.allocator);
      }
      PropertyKeyType::StringLiteral(s) => {
        self.string_keyed.insert(s.as_str(), keyed_property);
      }
      PropertyKeyType::NumericLiteral(n) => {
        let s = analyzer.allocator.alloc(n.0.to_js_string());
        self.string_keyed.insert(s, keyed_property);
      }
      PropertyKeyType::UniqueSymbol(s) => {
        self.symbol_keyed.insert(s, keyed_property);
      }
    }
  }

  pub fn init_spread(&mut self, analyzer: &mut Analyzer<'a>, value: Ty<'a>) {
    todo!()
  }

  pub fn delete_property(&mut self, analyzer: &mut Analyzer<'a>, value: Ty<'a>) {
    todo!()
  }
}

impl<'a> Analyzer<'a> {
  fn print_keyed_property(
    &self,
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
    &self,
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

  pub fn print_record_type(&self, record: &RecordType<'a>) -> TSType<'a> {
    let mut members = self.ast_builder.vec();
    for (key, property) in &record.string_keyed {
      members.push(
        self.print_keyed_property(
          self.ast_builder.property_key_identifier_name(SPAN, *key),
          property,
        ),
      );
    }
    for (key, property) in &record.symbol_keyed {
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
