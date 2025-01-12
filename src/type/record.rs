use crate::analyzer::Analyzer;

use super::Type;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
struct PropertyValue<'a> {
  value: Type<'a>,
  optional: bool,
  readonly: bool,
}

#[derive(Debug, Clone)]
pub struct Record<'a> {
  pub proto: Type<'a>,
  pub string_keyed: FxHashMap<&'a str, PropertyValue<'a>>,
  pub any_string_keyed: Option<PropertyValue<'a>>,
  pub any_number_keyed: Option<PropertyValue<'a>>,
}

impl<'a> Record<'a> {
  pub fn init_proto(&mut self, proto: Type<'a>) {
    self.proto = proto;
  }

  pub fn init_property(
    &mut self,
    analyzer: &mut Analyzer<'a>,
    key: Type<'a>,
    value: Type<'a>,
    optional: bool,
    readonly: bool,
  ) {
    self.string_keyed.insert(key, PropertyValue { value, optional, readonly });
  }

  pub fn init_spread(&mut self, analyzer: &mut Analyzer<'a>, value: Type<'a>) {
    self.any_string_keyed = Some(PropertyValue { value, optional: false, readonly: false });
  }

  pub fn delete_property(&mut self, analyzer: &mut Analyzer<'a>, key: Type<'a>) {
    self.string_keyed.remove(key);
  }
}

impl<'a> Analyzer<'a> {
  pub fn new_empty_record(&mut self) -> &'a mut Record<'a> {
    self.allocator.alloc(Record {
      proto: self.builtins.prototypes.object,
      string_keyed: FxHashMap::default(),
      any_string_keyed: None,
      any_number_keyed: None,
    })
  }
}
