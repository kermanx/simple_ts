use super::Type;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct Namespace<'a> {
  pub members: FxHashMap<&'a str, Type<'a>>,
}
