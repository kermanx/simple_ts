use super::Type;
use crate::{analyzer::Analyzer, utils::F64WithEq};
use oxc::{semantic::SymbolId, span::Atom};

#[derive(Debug, Clone, Copy)]
pub enum PropertyKeyType<'a> {
  Error,

  AnyString,
  AnyNumber,
  AnySymbol,

  StringLiteral(&'a Atom<'a>),
  NumericLiteral(F64WithEq),
  UniqueSymbol(SymbolId),
}

impl<'a> Analyzer<'a> {
  pub fn to_property_key(&mut self, from: Type<'a>) -> PropertyKeyType<'a> {
    match from {
      Type::Any => {
        // This is weird, but somehow TypeScript does this.
        PropertyKeyType::AnyNumber
      }
      Type::Unknown | Type::Never | Type::Void | Type::Error => PropertyKeyType::Error,

      Type::String => PropertyKeyType::AnyString,
      Type::Number => PropertyKeyType::AnyNumber,
      Type::Symbol => PropertyKeyType::AnySymbol,
      Type::BigInt | Type::Boolean | Type::Null | Type::Object | Type::Undefined => {
        PropertyKeyType::Error
      }

      Type::StringLiteral(s) => PropertyKeyType::StringLiteral(s),
      Type::NumericLiteral(n) => PropertyKeyType::NumericLiteral(n),
      Type::UniqueSymbol(s) => PropertyKeyType::UniqueSymbol(s),
      Type::BigIntLiteral(_) | Type::BooleanLiteral(_) => PropertyKeyType::Error,

      Type::Record(_) | Type::Function(_) | Type::Constructor(_) | Type::Namespace(_) => {
        PropertyKeyType::Error
      }

      Type::Union(values) => {
        let mut any_string = false;
        let mut any_number = false;
        let mut any_symbol = false;
        for value in values {
          match self.to_property_key(*value) {
            PropertyKeyType::Error => return PropertyKeyType::Error,
            PropertyKeyType::AnyString | PropertyKeyType::StringLiteral(_) => any_string = true,
            PropertyKeyType::AnyNumber | PropertyKeyType::NumericLiteral(_) => any_number = true,
            PropertyKeyType::AnySymbol | PropertyKeyType::UniqueSymbol(_) => any_symbol = true,
          }
        }
        match (any_string, any_number, any_symbol) {
          (true, _, _) | (false, true, true) => PropertyKeyType::AnyString,
          (false, true, false) => PropertyKeyType::AnyNumber,
          (false, false, true) => PropertyKeyType::AnySymbol,
          (false, false, false) => PropertyKeyType::Error,
        }
      }
      Type::Intersection(values) => {
        let mut any_string = false;
        let mut any_number = false;
        let mut any_symbol = false;
        for value in values {
          match self.to_property_key(*value) {
            PropertyKeyType::Error => return PropertyKeyType::Error,
            PropertyKeyType::AnyString => any_string = true,
            PropertyKeyType::AnyNumber => any_number = true,
            PropertyKeyType::AnySymbol => any_symbol = true,
            PropertyKeyType::StringLiteral(_)
            | PropertyKeyType::NumericLiteral(_)
            | PropertyKeyType::UniqueSymbol(_) => return PropertyKeyType::Error,
          }
        }
        match (any_string, any_number, any_symbol) {
          (true, false, false) => PropertyKeyType::AnyString,
          (false, true, false) => PropertyKeyType::AnyNumber,
          (false, false, true) => PropertyKeyType::AnySymbol,
          _ => PropertyKeyType::Error,
        }
      }

      Type::UnresolvedType(_) | Type::UnresolvedVariable(_) => todo!(),
      Type::Generic(_) | Type::Intrinsic(_) => unreachable!(),
    }
  }
}
