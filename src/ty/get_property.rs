use super::{property_key::PropertyKeyType, Ty};
use crate::Analyzer;

impl<'a> Analyzer<'a> {
  pub fn get_property(&mut self, target: Ty<'a>, key: PropertyKeyType<'a>) -> Ty<'a> {
    match target {
      Ty::Error | Ty::Any | Ty::Never | Ty::Unknown => target,

      Ty::Void | Ty::Null | Ty::Undefined => Ty::Error,

      Ty::Boolean | Ty::BooleanLiteral(_) => {
        self.get_property(self.builtins.boolean_prototype, key)
      }
      Ty::BigInt | Ty::BigIntLiteral(_) => self.get_property(self.builtins.bigint_prototype, key),
      Ty::Number | Ty::NumericLiteral(_) => self.get_property(self.builtins.number_prototype, key),
      Ty::Object => self.get_property(self.builtins.object_prototype, key),
      Ty::String | Ty::StringLiteral(_) => self.get_property(self.builtins.string_prototype, key),
      Ty::Symbol | Ty::UniqueSymbol(_) => self.get_property(self.builtins.symbol_prototype, key),
      Ty::Function(_) | Ty::Constructor(_) => {
        self.get_property(self.builtins.function_prototype, key)
      }

      Ty::Record(r) => self.get_record_property(r, key),
      Ty::Namespace(_) => todo!(),

      Ty::Union(u) => self.get_union_property(u, key),
      Ty::Intersection(_) => todo!(),

      Ty::Generic(_) | Ty::Intrinsic(_) => Ty::Error,

      Ty::Unresolved(u) => {
        if let Some(base) = self.get_unresolved_lowest_type(u) {
          self.get_property(base, key)
        } else {
          Ty::Error
        }
      }
    }
  }
}
