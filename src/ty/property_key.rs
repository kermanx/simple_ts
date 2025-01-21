use oxc::{semantic::SymbolId, span::Atom};

use super::Ty;
use crate::{analyzer::Analyzer, utils::F64WithEq};

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
  pub fn to_property_key(&mut self, from: Ty<'a>) -> PropertyKeyType<'a> {
    match from {
      Ty::Error => PropertyKeyType::Error,

      Ty::Any => {
        // This is weird, but somehow TypeScript does this.
        PropertyKeyType::AnyNumber
      }
      Ty::Unknown | Ty::Never | Ty::Void => PropertyKeyType::Error,

      Ty::String => PropertyKeyType::AnyString,
      Ty::Number => PropertyKeyType::AnyNumber,
      Ty::Symbol => PropertyKeyType::AnySymbol,
      Ty::BigInt | Ty::Boolean | Ty::Null | Ty::Object | Ty::Undefined => PropertyKeyType::Error,

      Ty::StringLiteral(s) => PropertyKeyType::StringLiteral(s),
      Ty::NumericLiteral(n) => PropertyKeyType::NumericLiteral(n),
      Ty::UniqueSymbol(s) => PropertyKeyType::UniqueSymbol(s),
      Ty::BigIntLiteral(_) | Ty::BooleanLiteral(_) => PropertyKeyType::Error,

      Ty::Record(_)
      | Ty::Function(_)
      | Ty::Constructor(_)
      | Ty::Interface(_)
      | Ty::Namespace(_) => PropertyKeyType::Error,

      Ty::Union(union) => {
        let mut has_error = false;
        let mut any_string = false;
        let mut any_number = false;
        let mut any_symbol = false;
        union.for_each(|ty| match self.to_property_key(ty) {
          PropertyKeyType::Error => has_error = true,
          PropertyKeyType::AnyString | PropertyKeyType::StringLiteral(_) => any_string = true,
          PropertyKeyType::AnyNumber | PropertyKeyType::NumericLiteral(_) => any_number = true,
          PropertyKeyType::AnySymbol | PropertyKeyType::UniqueSymbol(_) => any_symbol = true,
        });
        if has_error {
          PropertyKeyType::Error
        } else {
          match (any_string, any_number, any_symbol) {
            (true, _, _) | (false, true, true) => PropertyKeyType::AnyString,
            (false, true, false) => PropertyKeyType::AnyNumber,
            (false, false, true) => PropertyKeyType::AnySymbol,
            (false, false, false) => PropertyKeyType::Error,
          }
        }
      }
      Ty::Intersection(intersection) => {
        let mut has_error = false;
        let mut any_string = false;
        let mut any_number = false;
        let mut any_symbol = false;
        intersection.for_each(|ty| match self.to_property_key(ty) {
          PropertyKeyType::Error => has_error = true,
          PropertyKeyType::AnyString => any_string = true,
          PropertyKeyType::AnyNumber => any_number = true,
          PropertyKeyType::AnySymbol => any_symbol = true,
          PropertyKeyType::StringLiteral(_)
          | PropertyKeyType::NumericLiteral(_)
          | PropertyKeyType::UniqueSymbol(_) => has_error = true,
        });
        if has_error {
          PropertyKeyType::Error
        } else {
          match (any_string, any_number, any_symbol) {
            (true, false, false) => PropertyKeyType::AnyString,
            (false, true, false) => PropertyKeyType::AnyNumber,
            (false, false, true) => PropertyKeyType::AnySymbol,
            _ => PropertyKeyType::Error,
          }
        }
      }

      Ty::Generic(_) | Ty::Intrinsic(_) => PropertyKeyType::Error,

      Ty::Unresolved(u) => {
        if let Some(base) = self.get_unresolved_lowest_type(u) {
          self.to_property_key(base)
        } else {
          PropertyKeyType::Error
        }
      }
    }
  }
}
