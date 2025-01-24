use oxc::{allocator, ast::ast::Argument};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_arguments(
    &mut self,
    node: &'a allocator::Vec<'a, Argument<'a>>,
    mut sat: Option<Vec<(bool, Ty<'a>)>>,
  ) {
    for (i, arg) in node.iter().enumerate() {
      match arg {
        Argument::SpreadElement(node) => {
          self.exec_expression(&node.argument, None);
          sat = None;
        }
        node => {
          self.exec_expression(
            node.to_expression(),
            if let Some(s) = sat.as_ref() {
              if let Some((false, ty)) = s.get(i) {
                Some(*ty)
              } else {
                sat = None;
                None
              }
            } else {
              None
            },
          );
        }
      }
    }
  }
}
