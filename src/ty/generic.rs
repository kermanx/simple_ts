use super::Ty;
use crate::analyzer::Analyzer;
use oxc::{ast::ast::TSType, semantic::SymbolId};
use rustc_hash::FxHashMap;
use std::mem;

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
  pub fn take_generics(&mut self) -> Box<FxHashMap<SymbolId, Ty<'a>>> {
    mem::take(&mut self.generics)
  }

  pub fn restore_generics(&mut self, generics: Box<FxHashMap<SymbolId, Ty<'a>>>) {
    self.generics = generics;
  }

  pub fn instantiate_generic_param(&mut self, params: &Vec<GenericParam<'a>>, args: Vec<Ty<'a>>) {
    for (index, param) in params.iter().enumerate() {
      let arg = args.get(index).copied().or(param.default).unwrap_or(Ty::Error);
      self.generics.insert(param.symbol_id, arg);
    }
    for param in params.iter() {
      if let Some(constraint) = param.constraint {
        // TODO: Check constraint
      }
    }
  }

  pub fn instantiate_generic(&mut self, ty: Ty<'a>, args: Vec<Ty<'a>>) -> Option<Ty<'a>> {
    match ty {
      Ty::Generic(generic) => {
        let old_generics = self.take_generics();
        self.instantiate_generic_param(&generic.params, args);
        let result = self.resolve_type(generic.body);
        self.restore_generics(old_generics);
        result
      }
      _ => unreachable!("Cannot instantiate non-generic type"),
    }
  }

  pub fn print_generic_type(&self, generic: &GenericType<'a>) -> TSType<'a> {
    todo!()
  }
}
