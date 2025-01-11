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
  string_keyed: FxHashMap<&'a str, PropertyValue<'a>>,
  any_string_keyed: Option<PropertyValue<'a>>,
  any_number_keyed: Option<PropertyValue<'a>>,
}
