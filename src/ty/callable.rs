use oxc::{
  allocator,
  ast::{
    ast::{Argument, FormalParameterKind, TSType, TSTypeParameterInstantiation},
    NONE,
  },
  semantic::SymbolId,
  span::SPAN,
};

use super::{generic::GenericParam, union::UnionType, Ty};
use crate::analyzer::Analyzer;

#[derive(Debug, Clone)]
pub struct CallableType<'a, const CTOR: bool> {
  /// Method is bivariant
  pub bivariant: bool,

  pub type_params: Vec<GenericParam<'a>>,
  pub this_param: Option<Ty<'a>>,
  /// (optional, type)
  pub params: Vec<(bool, &'a TSType<'a>)>,
  pub rest_param: Option<&'a TSType<'a>>,
  pub return_type: &'a TSType<'a>,
}

pub type FunctionType<'a> = CallableType<'a, false>;
pub type ConstructorType<'a> = CallableType<'a, true>;

impl<'a> Analyzer<'a> {
  pub fn instantiate_callable_type_parameters<const CTOR: bool>(
    &mut self,
    callable: &CallableType<'a, CTOR>,
    type_args: &Vec<Ty<'a>>,
  ) -> Option<&'a CallableType<'a, CTOR>> {
    if callable.type_params.len() != type_args.len() {
      return None;
    }

    let old_generics = self.take_generics();
    self.instantiate_generic_params(&callable.type_params, type_args);
    let this_type = callable.this_param.map(|ty| self.resolve_unresolved(ty));
    let params = callable
      .params
      .iter()
      .map(|(optional, ty)| (*optional, self.resolve_unresolved(*ty)))
      .collect();
    let rest_param = callable.rest_param.map(|ty| self.resolve_unresolved(ty));
    let return_type = self.resolve_unresolved(callable.return_type);
    self.restore_generics(old_generics);
    Some(self.allocator.alloc(CallableType {
      bivariant: callable.bivariant,
      type_params: vec![],
      this_param: this_type,
      params,
      rest_param,
      return_type,
    }))
  }

  pub fn print_callable_type<const CTOR: bool>(
    &mut self,
    callable: &CallableType<'a, CTOR>,
  ) -> TSType<'a> {
    self.ast_builder.ts_type_function_type(
      SPAN,
      /* TODO: */ NONE,
      callable.this_param.map(|ty| {
        self.ast_builder.ts_this_parameter(
          SPAN,
          SPAN,
          Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(ty))),
        )
      }),
      self.ast_builder.formal_parameters(
        SPAN,
        FormalParameterKind::Signature,
        {
          let mut items = self.ast_builder.vec();
          for (i, (optional, param)) in callable.params.iter().enumerate() {
            items.push(self.ast_builder.formal_parameter(
              SPAN,
              self.ast_builder.vec(),
              self.ast_builder.binding_pattern(
                self.ast_builder.binding_pattern_kind_binding_identifier(
                  SPAN,
                  &*self.allocator.alloc(format!("a{i}")),
                ),
                Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(*param))),
                *optional,
              ),
              None,
              false,
              false,
            ))
          }
          if let Some(rest) = callable.rest_param {}
          items
        },
        callable.rest_param.map(|ty| {
          self.ast_builder.binding_rest_element(
            SPAN,
            self.ast_builder.binding_pattern(
              self.ast_builder.binding_pattern_kind_binding_identifier(SPAN, "rest"),
              Some(self.ast_builder.ts_type_annotation(SPAN, self.print_type(ty))),
              false,
            ),
          )
        }),
      ),
      self.ast_builder.ts_type_annotation(SPAN, self.print_type(callable.return_type)),
    )
  }
}

#[derive(Debug)]
pub enum ExtractedCallable<'a, const CTOR: bool> {
  Single(&'a CallableType<'a, CTOR>),
  Overloaded(Vec<ExtractedCallable<'a, CTOR>>),
  Union(Vec<ExtractedCallable<'a, CTOR>>),
}

macro_rules! impl_extract_callable {
  ($name: ident, $ctor: expr, $member: ident) => {
    impl<'a> Analyzer<'a> {
      pub fn $name(&mut self, ty: Ty<'a>) -> Option<ExtractedCallable<'a, $ctor>> {
        match ty {
          Ty::$member(f) => Some(ExtractedCallable::Single(f)),
          Ty::Union(u) => {
            let mut res = Some(vec![]);
            u.for_each(|ty| {
              if let (Some(res), Some(extracted)) = (&mut res, self.$name(ty)) {
                res.push(extracted);
              } else {
                res = None;
              }
            });
            res.map(ExtractedCallable::Union)
          }
          Ty::Intersection(i) => {
            let mut res = vec![];
            i.for_each(|ty| {
              if let Some(extracted) = self.$name(ty) {
                res.push(extracted);
              }
            });
            match res.len() {
              0 => None,
              1 => Some(res.into_iter().next().unwrap()),
              _ => Some(ExtractedCallable::Overloaded(res)),
            }
          }
          _ => None,
        }
      }
    }
  };
}

impl_extract_callable!(extract_callable_function, false, Function);
impl_extract_callable!(extract_callable_constructor, true, Constructor);

impl<'a> Analyzer<'a> {
  pub fn get_callable_parameter_types<const CTOR: bool>(
    &mut self,
    callable: &ExtractedCallable<'a, CTOR>,
  ) -> Vec<Ty<'a>> {
    match callable {
      ExtractedCallable::Single(callable) => callable
        .params
        .iter()
        .copied()
        .map(|(optional, ty)| self.get_optional_type(optional, ty))
        .collect(),
      ExtractedCallable::Overloaded(callables) => {
        let allocator = self.allocator;
        let callables = callables.iter().map(|c| self.get_callable_parameter_types(c));
        let mut res = Vec::new();
        for callable in callables {
          for _ in res.len()..callable.len() {
            res.push(allocator.alloc(UnionType::default()));
          }
          for (i, item) in callable.into_iter().enumerate() {
            res[i].add(item);
          }
        }
        res.into_iter().map(|u| Ty::Union(u)).collect()
      }
      ExtractedCallable::Union(callables) => {
        todo!()
      }
    }
  }

  pub fn exec_call<const CTOR: bool>(
    &mut self,
    callable: Option<ExtractedCallable<'a, CTOR>>,
    type_parameters: &'a Option<allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
    this_arg: Ty<'a>,
    arguments: &'a allocator::Vec<'a, Argument<'a>>,
    ret_sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    if let Some(callable) = callable {
      match callable {
        ExtractedCallable::Single(callable) => {
          if callable.type_params.is_empty() {
            let params = self.get_callable_parameter_types(&ExtractedCallable::Single(callable));
            self.exec_arguments(arguments, params);
            callable.return_type
          } else if let Some(type_parameters) = type_parameters {
            let type_args = self.resolve_type_parameter_instantiation(type_parameters);
            let old_generics = self.take_generics();
            self.instantiate_generic_params(&callable.type_params, &type_args);
            let params = self.get_callable_parameter_types(&ExtractedCallable::Single(callable));
            self.exec_arguments(arguments, params);
            let ret = self.resolve_unresolved(callable.return_type);
            self.restore_generics(old_generics);
            ret
          } else {
            todo!()
          }
        }
        ExtractedCallable::Overloaded(callables) => {
          let ret = Ty::Error;
          for callable in callables {
            todo!();
            // If matches, set ret_val and break
          }
          ret
        }
        ExtractedCallable::Union(callables) => {
          todo!()
        }
      }
    } else {
      for arg in arguments {
        match arg {
          Argument::SpreadElement(node) => {
            self.exec_expression(&node.argument, None);
          }
          node => {
            self.exec_expression(node.to_expression(), None);
          }
        }
      }
      Ty::Error
    }
  }
}
