use crate::{analyzer::Analyzer, ty::{record::Record, Ty}};

impl<'a> Analyzer<'a> {
  /// const { enumerated_1, enumerated_2, ...rest } = object;
  pub fn exec_object_rest(&mut self, object: Ty<'a>, enumerated: Vec<Ty<'a>>) -> Ty<'a> {
    let rest = self.allocator.alloc(Record::default());

    rest.init_spread(self, object);

    for key in enumerated {
      rest.delete_property(self, key);
    }

    Ty::Record(rest)
  }
}
