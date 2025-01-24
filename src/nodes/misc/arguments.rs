use oxc::{allocator, ast::ast::Argument};

use crate::{analyzer::Analyzer, ty::Ty};

impl<'a> Analyzer<'a> {
  pub fn exec_arguments(
    &mut self,
    node: &'a allocator::Vec<'a, Argument<'a>>,
    sat: Option<Vec<Ty<'a>>>,
  ) {
    let mut in_order = true;
    for (i, arg) in node.iter().enumerate() {
      match arg {
        Argument::SpreadElement(node) => {
          self.exec_expression(&node.argument, None);
          in_order = false;
        }
        node => {
          self.exec_expression(
            node.to_expression(),
            sat.as_ref().and_then(|sat| in_order.then(|| sat.get(i).copied().unwrap_or(Ty::Error))),
          );
        }
      }
    }
  }
}
