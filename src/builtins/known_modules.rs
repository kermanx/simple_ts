use super::Builtins;
use crate::r#type::Type;

#[derive(Debug, Clone, Copy)]
pub struct KnownModule<'a> {
  pub namespace: Type<'a>,
  pub default: Type<'a>,
}

impl<'a> Builtins<'a> {
  pub fn init_known_modules(&mut self) {}

  pub fn get_known_module(&self, name: &str) -> Option<KnownModule<'a>> {
    let name = name.strip_prefix("https://esm.sh/").unwrap_or(name);
    self.known_modules.get(name).copied()
  }
}
