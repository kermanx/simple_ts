use super::{object::create_object_prototype, Prototype};
use crate::{
  entity::{Entity, EntityFactory},
  init_prototype,
};
use oxc::semantic::SymbolId;
use oxc_index::Idx;

pub fn create_function_prototype<'a>(factory: &EntityFactory<'a>) -> Prototype<'a> {
  init_prototype!("Function", create_object_prototype(factory), {
    "apply" => factory.implemented_builtin_fn("Function::apply", |analyzer, this, args| {
      let mut args = args.destruct_as_array(analyzer, 2, false).0;
      let args_arg = {
        let arg = args.pop().unwrap();
        let cf_scope = analyzer.scope_context.cf.current_id();
        // This can be any value
        let arguments_object_id = SymbolId::from_usize(0);
        match arg.test_is_undefined() {
          Some(true) => analyzer.factory.array(cf_scope, arguments_object_id),
          Some(false) => arg,
          None => analyzer.factory.union((
            arg,
            analyzer.factory.array(cf_scope, arguments_object_id) as Entity<'a>,
          )),
        }
      };
      let this_arg = args.pop().unwrap();
      this.call(analyzer, this_arg, args_arg)
    }),
    "call" => factory.implemented_builtin_fn("Function::call", |analyzer, this, args| {
      let (this_arg, args_arg) = args.destruct_as_array(analyzer, 1, true);
      this.call(analyzer, this_arg[0], args_arg.unwrap())
    }),
    "bind" => factory.pure_fn_returns_unknown,
    "length" => factory.unknown_number,
    "arguments" => factory.unknown,
    "caller" => factory.unknown,
    "name" => factory.string,
    "prototype" => factory.unknown,
  })
}
