use crate::{ty::Ty, Analyzer};
use oxc::ast::ast::TSThisParameter;

impl<'a> Analyzer<'a> {
  pub fn resovle_this_parameter(&mut self, node: &'a TSThisParameter<'a>) -> Ty<'a> {
    if let Some(type_annotation) = &node.type_annotation {
      self.resolve_type_annotation(type_annotation)
    } else {
      Ty::Any
    }
  }
}
