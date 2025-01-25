use crate::{
  analyzer::Analyzer,
  ty::{property_key::PropertyKeyType, record::RecordTypeBuilder, Ty},
};

impl<'a> Analyzer<'a> {
  /// const { enumerated_1, enumerated_2, ...rest } = object;
  pub fn exec_object_rest(
    &mut self,
    object: Ty<'a>,
    enumerated: Vec<PropertyKeyType<'a>>,
  ) -> Ty<'a> {
    let mut rest = RecordTypeBuilder::default();

    rest.init_spread(self, object);

    for key in enumerated {
      rest.remove_property(self, key);
    }

    Ty::Record(self.allocator.alloc(rest.build()))
  }
}
