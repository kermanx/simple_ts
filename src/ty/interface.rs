use super::{record::RecordType, Ty};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct InterfaceType<'a> {
  pub record: RefCell<RecordType<'a>>,
  pub callables: RefCell<Vec<Ty<'a>>>,
}
