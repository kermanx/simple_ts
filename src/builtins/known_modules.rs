use super::Builtins;
use crate::entity::Entity;

#[derive(Debug, Clone, Copy)]
pub struct KnownModule<'a> {
  pub namespace: Entity<'a>,
  pub default: Entity<'a>,
}

impl<'a> Builtins<'a> {
  pub fn init_known_modules(&mut self) {}

  pub fn get_known_module(&self, name: &str) -> Option<KnownModule<'a>> {
    let name = name.strip_prefix("https://esm.sh/").unwrap_or(name);
    self.known_modules.get(name).copied()
  }
}
