use oxc::ast::ast::TSInstantiationExpression;

use crate::{Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_ts_instantiation_expression(
    &mut self,
    node: &'a TSInstantiationExpression<'a>,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    let base = self.exec_expression(&node.expression, None);
    let type_args = self.resolve_type_parameter_instantiation(&node.type_arguments);
    self.instantiate_generic_value(base, &type_args)
  }
}
