use super::{
  callable::{ConstructorType, FunctionType},
  record::RecordType,
};
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct InterfaceType<'a> {
  pub record: RefCell<RecordType<'a>>,
  pub functions: RefCell<Vec<FunctionType<'a>>>,
  pub constructors: RefCell<Vec<ConstructorType<'a>>>,
}
