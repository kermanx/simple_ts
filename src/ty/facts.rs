use super::Ty;
use crate::analyzer::Analyzer;
use bitflags::bitflags;

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Facts: u32 {
    const NONE = 0;

    const T_EQ_STRING = 1 << 0;
    const T_EQ_NUMBER = 1 << 1;
    const T_EQ_BIGINT = 1 << 2;
    const T_EQ_BOOLEAN = 1 << 3;
    const T_EQ_SYMBOL = 1 << 4;
    const T_EQ_OBJECT = 1 << 5;
    const T_EQ_FUNCTION = 1 << 6;

    const T_NE_STRING = 1 << 7;
    const T_NE_NUMBER = 1 << 8;
    const T_NE_BIGINT = 1 << 9;
    const T_NE_BOOLEAN = 1 << 10;
    const T_NE_SYMBOL = 1 << 11;
    const T_NE_OBJECT = 1 << 12;
    const T_NE_FUNCTION = 1 << 13;

    const EQ_NULL = 1 << 14;
    const EQ_UNDEFINED = 1 << 15;

    const NE_NULL = 1 << 16;
    const NE_UNDEFINED = 1 << 17;

    const IS_NULLISH = 1 << 18;
    const NOT_NULLISH = 1 << 19;

    const TRUTHY = 1 << 20;
    const FALSY = 1 << 21;

    const T_NE_ALL = Self::T_NE_STRING.bits()
      | Self::T_NE_NUMBER.bits()
      | Self::T_NE_BIGINT.bits()
      | Self::T_NE_BOOLEAN.bits()
      | Self::T_NE_SYMBOL.bits()
      | Self::T_NE_OBJECT.bits()
      | Self::T_NE_FUNCTION.bits()
      | Self::NE_NULL.bits()
      | Self::NE_UNDEFINED.bits()
      | Self::NOT_NULLISH.bits();
  }
}

impl Facts {
  pub fn truthy(truthy: bool) -> Self {
    if truthy {
      Self::TRUTHY
    } else {
      Self::FALSY
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn get_facts(&mut self, ty: Ty<'a>) -> Facts {
    match ty {
      Ty::Error => Facts::NONE,

      Ty::Any => Facts::NONE,
      Ty::Unknown => Facts::NONE,
      Ty::Never => Facts::T_NE_ALL,
      Ty::Void => Facts::FALSY | Facts::T_NE_ALL,

      Ty::BigInt => Facts::T_EQ_BIGINT | Facts::T_NE_ALL & !Facts::T_EQ_BIGINT,
      Ty::Boolean => Facts::T_EQ_BOOLEAN | Facts::T_NE_ALL & !Facts::T_EQ_BOOLEAN,
      Ty::Null => {
        Facts::EQ_NULL
          | Facts::IS_NULLISH
          | Facts::FALSY
          | Facts::T_EQ_OBJECT
          | Facts::T_NE_ALL & !Facts::NE_NULL & !Facts::T_NE_OBJECT
      }
      Ty::Number => Facts::T_EQ_NUMBER | Facts::T_NE_ALL & !Facts::T_EQ_NUMBER,
      Ty::Object => Facts::T_EQ_OBJECT | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_OBJECT,
      Ty::String => Facts::T_EQ_STRING | Facts::T_NE_ALL & !Facts::T_EQ_STRING,
      Ty::Symbol => Facts::T_EQ_SYMBOL | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_SYMBOL,
      Ty::Undefined => {
        Facts::EQ_UNDEFINED
          | Facts::IS_NULLISH
          | Facts::FALSY
          | Facts::T_NE_ALL & !Facts::NE_UNDEFINED
      }

      Ty::StringLiteral(s) => self.get_facts(Ty::String) | Facts::truthy(s.len() > 0),
      Ty::NumericLiteral(n) => self.get_facts(Ty::Number) | Facts::truthy(n.0 != 0.0),
      Ty::BigIntLiteral(_) => self.get_facts(Ty::BigInt),
      Ty::BooleanLiteral(b) => self.get_facts(Ty::Boolean) | Facts::truthy(b),
      Ty::UniqueSymbol(_) => self.get_facts(Ty::Symbol),

      Ty::Record(_) => self.get_facts(Ty::Object),
      Ty::Function(_) | Ty::Constructor(_) => {
        Facts::T_EQ_FUNCTION | Facts::TRUTHY | Facts::T_NE_ALL & !Facts::T_EQ_FUNCTION
      }
      Ty::Interface(_) | Ty::Namespace(_) => self.get_facts(Ty::Object),

      Ty::Union(union) => {
        let mut facts = Facts::all();
        union.for_each(|ty| facts &= self.get_facts(ty));
        facts
      }
      Ty::Intersection(intersection) => {
        let mut facts = Facts::empty();
        intersection.for_each(|ty| facts |= self.get_facts(ty));
        facts
      }

      Ty::Generic(_) | Ty::Intrinsic(_) => {
        unreachable!("Cannot get facts of {ty:?}")
      }

      Ty::Unresolved(u) => {
        if let Some(base) = self.get_unresolved_lowest_type(u) {
          self.get_facts(base)
        } else {
          Facts::NONE
        }
      }
    }
  }

  pub fn test_truthy(&mut self, ty: Ty<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::TRUTHY), facts.contains(Facts::FALSY)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("TRUTHY and FALSY are mutually exclusive"),
    }
  }

  pub fn test_nullish(&mut self, ty: Ty<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::IS_NULLISH), facts.contains(Facts::NOT_NULLISH)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("IS_NULLISH and NOT_NULLISH are mutually exclusive"),
    }
  }

  pub fn test_is_undefined(&mut self, ty: Ty<'a>) -> Option<bool> {
    let facts = self.get_facts(ty);
    match (facts.contains(Facts::EQ_UNDEFINED), facts.contains(Facts::NE_UNDEFINED)) {
      (true, false) => Some(true),
      (false, true) => Some(false),
      (false, false) => None,
      (true, true) => unreachable!("EQ_UNDEFINED and NE_UNDEFINED are mutually exclusive"),
    }
  }
}
