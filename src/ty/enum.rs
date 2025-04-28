use oxc::{
  ast::{NONE, ast::TSType},
  span::{Atom, SPAN},
};
use oxc_index::define_index_type;

use crate::Analyzer;

use super::{Ty, property_key::PropertyKeyType, record::RecordType};

define_index_type! {
  pub struct EnumClassId = u32;
}

#[derive(Debug)]
pub struct EnumClassType<'a> {
  pub id: EnumClassId,
  pub name: Atom<'a>,
  pub record: &'a RecordType<'a>,
  // `E[1]` is allowed
  pub number_value: bool,
  // `E["value"]` is allowed
  pub string_value: bool,
}

impl<'a> EnumClassType<'a> {
  pub fn record_ty(&self) -> Ty<'a> {
    Ty::Record(self.record)
  }

  pub fn get_property(&self, key: PropertyKeyType<'a>, analyzer: &mut Analyzer<'a>) -> Ty<'a> {
    match key {
      PropertyKeyType::Error
      | PropertyKeyType::AnyString
      | PropertyKeyType::UniqueSymbol(_)
      | PropertyKeyType::AnySymbol => Ty::Error,
      PropertyKeyType::NumericLiteral(_) | PropertyKeyType::AnyNumber => {
        if self.number_value {
          Ty::String
        } else {
          Ty::Error
        }
      }
      PropertyKeyType::StringLiteral(_) => analyzer.get_property(self.record_ty(), key),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumMemberType<'a> {
  pub class: EnumClassId,
  pub name: Option<Atom<'a>>,
  pub value: Ty<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn serialize_enum_class_type(&mut self, enum_class: &'a EnumClassType<'a>) -> TSType<'a> {
    todo!("enum T")
  }

  pub fn serialize_enum_member_type(&mut self, enum_member: &'a EnumMemberType<'a>) -> TSType<'a> {
    let enum_class = self
      .ast_builder
      .ts_type_name_identifier_reference(SPAN, self.enum_classes[enum_member.class].name);
    self.ast_builder.ts_type_type_reference(
      SPAN,
      if let Some(name) = enum_member.name {
        self.ast_builder.ts_type_name_qualified_name(
          SPAN,
          enum_class,
          self.ast_builder.identifier_name(SPAN, name),
        )
      } else {
        enum_class
      },
      NONE,
    )
  }
}
