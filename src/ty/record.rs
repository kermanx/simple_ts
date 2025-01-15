use super::{property_key::PropertyKeyType, Ty};
use crate::analyzer::Analyzer;
use oxc::semantic::SymbolId;
use oxc_syntax::number::ToJsString;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
struct KeyedProperty<'a> {
  value: Ty<'a>,
  optional: bool,
  readonly: bool,
}

#[derive(Debug, Default, Clone)]
struct MappedProperty<'a> {
  values: Vec<Ty<'a>>,
  readonly: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Record<'a> {
  pub proto: Option<Ty<'a>>,

  pub string_keyed: FxHashMap<&'a str, KeyedProperty<'a>>,
  pub symbol_keyed: FxHashMap<SymbolId, KeyedProperty<'a>>,

  pub string_mapped: MappedProperty<'a>,
  pub number_mapped: MappedProperty<'a>,
  pub symbol_mapped: MappedProperty<'a>,
}

impl<'a> Record<'a> {
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
        self.string_mapped.values.push(value);
      }
      PropertyKeyType::AnyNumber => {
        self.number_mapped.values.push(value);
      }
      PropertyKeyType::AnySymbol => {
        self.symbol_mapped.values.push(value);
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
