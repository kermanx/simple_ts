#[derive(Debug, Clone)]
pub struct Config {
  pub unmatched_prototype_property_as_undefined: bool,
  pub max_recursion_depth: usize,
}

impl Default for Config {
  fn default() -> Self {
    Self { unmatched_prototype_property_as_undefined: true, max_recursion_depth: 2 }
  }
}
