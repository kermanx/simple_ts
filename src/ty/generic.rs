use super::Ty;
use crate::analyzer::Analyzer;
use oxc::{ast::ast::TSType, semantic::SymbolId};

#[derive(Debug, Clone)]
pub struct GenericParam<'a> {
  pub symbol_id: SymbolId,
  pub constraint: Option<Ty<'a>>,
  pub default: Option<Ty<'a>>,
  pub r#in: bool,
  pub out: bool,
  pub r#const: bool,
}

#[derive(Debug, Clone)]
pub struct GenericType<'a> {
  pub params: Vec<GenericParam<'a>>,
  pub body: &'a TSType<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn instantiate_generic(&mut self, ty: Ty<'a>, args: Vec<Ty<'a>>) -> Option<Ty<'a>> {
    match ty {
      Ty::Generic(generic) => {
        self.push_variable_scope();
        for (index, param) in generic.params.iter().enumerate() {
          let arg = args.get(index).copied().or(param.default).unwrap_or(Ty::Error);
          self.types.insert(param.symbol_id, arg);
        }
        for param in generic.params.iter() {
          if let Some(constraint) = param.constraint {
            // TODO: Check constraint
          }
        }
        let result = self.resolve_type(generic.body);
        self.pop_variable_scope();
        result
      }
      _ => unreachable!("Cannot instantiate non-generic type"),
    }
  }

  pub fn print_generic_type(&self, generic: &GenericType<'a>) -> TSType<'a> {
    todo!()
  }
}
