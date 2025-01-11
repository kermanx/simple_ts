use super::Prototype;
use crate::r#type::EntityFactory;

pub fn create_null_prototype<'a>(_factory: &EntityFactory<'a>) -> Prototype<'a> {
  Prototype::default().with_name("null")
}
