mod globals;

use oxc::{parser::Parser, span::SourceType};

use crate::{
  allocator::Allocator,
  ty::{Ty, namespace::Ns},
};

pub struct Builtins<'a> {
  pub lib_es5: &'a Ns<'a>,
  pub boolean_prototype: Ty<'a>,
  pub bigint_prototype: Ty<'a>,
  pub number_prototype: Ty<'a>,
  pub object_prototype: Ty<'a>,
  pub string_prototype: Ty<'a>,
  pub symbol_prototype: Ty<'a>,
  pub function_prototype: Ty<'a>,
}

impl<'a> Builtins<'a> {
  pub fn new(allocator: Allocator<'a>) -> Self {
    // 暂时使用一个空的 namespace，稍后需要实现真正的解析
    let lib_es5 = allocator.alloc(Ns::new_in(allocator));
    Self {
      lib_es5,
      boolean_prototype: Ty::Object,
      bigint_prototype: Ty::Object,
      number_prototype: Ty::Object,
      object_prototype: Ty::Object,
      string_prototype: Ty::Object,
      symbol_prototype: Ty::Object,
      function_prototype: Ty::Object,
    }
  }

  pub fn parse(allocator: Allocator<'a>, source_text: &'a str) -> &'a Ns<'a> {
    let parser = Parser::new(*allocator, source_text, SourceType::d_ts());
    let program = parser.parse();
    // 暂时返回一个空的 namespace
    allocator.alloc(Ns::new_in(allocator))
  }
}
