use oxc::{
  allocator,
  ast::{
    ast::{Argument, FormalParameterKind, TSType, TSTypeParameterInstantiation},
    NONE,
  },
  semantic::SymbolId,
  span::SPAN,
};
use rustc_hash::FxHashMap;

use super::{ctx::CtxTy, generic::GenericParam, Ty};
use crate::{
  analyzer::Analyzer,
  scope::r#type::TypeScopeId,
  ty::{r#match::MatchResult, unresolved::UnresolvedType},
};

#[derive(Debug, Clone)]
pub struct CallableType<'a, const CTOR: bool> {
  pub is_method: bool,
  pub scope: TypeScopeId,

  pub type_params: Vec<GenericParam<'a>>,
  pub this_param: Option<CtxTy<'a>>,
  /// (optional, type)
  pub params: Vec<(bool, CtxTy<'a>)>,
  pub rest_param: Option<CtxTy<'a>>,
  pub return_type: CtxTy<'a>,
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

    let scope = self.instantiate_generic_params(&callable.type_params, type_args);
    self.type_scopes.set_parent(scope, callable.scope);
    let this_type = callable.this_param.map(|ty| ty.with_scope(scope));
    let params =
      callable.params.iter().map(|(optional, ty)| (*optional, ty.with_scope(scope))).collect();
    let rest_param = callable.rest_param.map(|ty| ty.with_scope(scope));
    let return_type = callable.return_type.with_scope(scope);
    Some(self.allocator.alloc(CallableType {
      is_method: callable.is_method,
      scope,
      type_params: vec![],
      this_param: this_type,
      params,
      rest_param,
      return_type,
    }))
  }

  pub fn serialize_callable_type<const CTOR: bool>(
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
          Some(self.ast_builder.ts_type_annotation(SPAN, self.serialize_ctx_ty(ty))),
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
                Some(self.ast_builder.ts_type_annotation(SPAN, self.serialize_ctx_ty(*param))),
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
              Some(self.ast_builder.ts_type_annotation(SPAN, self.serialize_ctx_ty(ty))),
              false,
            ),
          )
        }),
      ),
      self.ast_builder.ts_type_annotation(SPAN, self.serialize_ctx_ty(callable.return_type)),
    )
  }
}

#[derive(Debug)]
pub enum ExtractedCallable<'a, const CTOR: bool> {
  Any,
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
          Ty::Any => Some(ExtractedCallable::Any),
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
          Ty::Instance(i) => {
            let unwrapped = self.unwrap_generic_instance(i);
            self.$name(unwrapped)
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
    scope: TypeScopeId,
    callable: &ExtractedCallable<'a, CTOR>,
  ) -> Vec<(bool, Ty<'a>)> {
    match callable {
      ExtractedCallable::Any => vec![(true, Ty::Any)],
      ExtractedCallable::Single(callable) => callable
        .params
        .iter()
        .copied()
        .map(|(optional, ty)| {
          let ty = self.resolve_ctx_ty(scope, ty);
          (false, self.get_optional_type(optional, ty))
        })
        .collect(),
      ExtractedCallable::Overloaded(callables) => {
        let res = self.get_transposed_callable_parameter_types(scope, callables);
        res.into_iter().map(|u| (false, self.into_union(u).unwrap())).collect()
      }
      ExtractedCallable::Union(callables) => {
        let res = self.get_transposed_callable_parameter_types(scope, callables);
        res.into_iter().map(|u| (false, self.into_intersection(u))).collect()
      }
    }
  }

  fn get_transposed_callable_parameter_types<const CTOR: bool>(
    &mut self,
    scope: TypeScopeId,
    callables: &Vec<ExtractedCallable<'a, CTOR>>,
  ) -> Vec<Vec<Ty<'a>>> {
    let callables = callables.iter().map(|c| self.get_callable_parameter_types(scope, c));
    let mut res = Vec::new();
    for callable in callables {
      for _ in res.len()..callable.len() {
        res.push(Vec::new());
      }
      for (i, (spread, item)) in callable.into_iter().enumerate() {
        if spread {
          todo!()
        } else {
          res[i].push(item);
        }
      }
    }
    res
  }

  /// Returns `None` if the signature does not match. Otherwise, returns the return type.
  pub fn exec_call<const CTOR: bool>(
    &mut self,
    callable: Option<ExtractedCallable<'a, CTOR>>,
    type_args: &'a Option<allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
    this_arg: Ty<'a>,
    arguments: &'a allocator::Vec<'a, Argument<'a>>,
    ret_sat: Option<Ty<'a>>,
  ) -> Option<Ty<'a>> {
    if let Some(callable) = callable {
      match callable {
        ExtractedCallable::Any => {
          self.exec_arguments(arguments, None);
          None
        }
        ExtractedCallable::Single(callable) => {
          self.exec_call_on_single(callable, type_args, this_arg, arguments, ret_sat)
        }
        ExtractedCallable::Overloaded(callables) => {
          for callable in callables {
            if let Some(ret) =
              self.exec_call(Some(callable), type_args, this_arg, arguments, ret_sat)
            {
              return Some(ret);
            }
          }
          None
        }
        ExtractedCallable::Union(callables) => {
          let mut ret_types = Vec::new();
          for callable in callables {
            let ret = self.exec_call(Some(callable), type_args, this_arg, arguments, ret_sat)?;
            ret_types.push(ret);
          }
          self.into_union(ret_types)
        }
      }
    } else {
      self.exec_arguments(arguments, None);
      None
    }
  }

  fn exec_call_on_single<const CTOR: bool>(
    &mut self,
    callable: &'a CallableType<'a, CTOR>,
    type_args: &'a Option<allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
    this_arg: Ty<'a>,
    arguments: &'a allocator::Vec<'a, Argument<'a>>,
    ret_sat: Option<Ty<'a>>,
  ) -> Option<Ty<'a>> {
    if callable.type_params.is_empty() {
      // Not generic
      let params = self.get_callable_parameter_types(
        self.type_scopes.empty_scope,
        &ExtractedCallable::Single(callable),
      );
      self.exec_arguments(arguments, Some(params));
      Some(self.resolve_ctx_ty(self.type_scopes.empty_scope, callable.return_type))
    } else if let Some(type_args) = type_args {
      // Generic, and type arguments are provided
      let type_args = self.resolve_type_parameter_instantiation(type_args);
      let scope = self.instantiate_generic_params(&callable.type_params, &type_args);
      let params = self.get_callable_parameter_types(scope, &ExtractedCallable::Single(callable));
      self.exec_arguments(arguments, Some(params));
      Some(self.resolve_ctx_ty(scope, callable.return_type))
    } else {
      // Generic, and need inference
      self.exec_call_with_inference(callable, this_arg, arguments, ret_sat)
    }
  }

  fn exec_call_with_inference<const CTOR: bool>(
    &mut self,
    callable: &'a CallableType<'a, CTOR>,
    this_arg: Ty<'a>,
    arguments: &'a allocator::Vec<'a, Argument<'a>>,
    ret_sat: Option<Ty<'a>>,
  ) -> Option<Ty<'a>> {
    // # Inference
    // See https://gitnation.com/contents/lets-make-a-generic-inference-algorithm
    //
    // - Non-context-aware arguments first (TODO)
    // - Type inferred from input type is the upper-bound (specificity < 0)
    // - Type inferred from output type is the lower-bound (specificity > 0)
    // - Choose the widest type *FROM* output type inferred from output type

    let scope = self.type_scopes.create_scope();
    for param in &callable.type_params {
      self.type_scopes.insert_on_scope(
        scope,
        param.symbol_id,
        Ty::Unresolved(UnresolvedType::InferType(param.symbol_id)),
      );
    }
    let params = self.get_callable_parameter_types(scope, &ExtractedCallable::Single(callable));

    #[derive(Default)]
    struct InferenceState<'a> {
      upmost_output: Option<Ty<'a>>,
      lowest_input: Option<Ty<'a>>,
    }

    impl<'a> InferenceState<'a> {
      pub fn update(&mut self, analyzer: &mut Analyzer<'a>, specificity: i32, ty: Ty<'a>) {
        if specificity > 0 {
          if let Some(upmost) = &mut self.upmost_output {
            if analyzer.match_covariant_types(1, *upmost, ty).matched() {
              *upmost = ty;
            }
          } else {
            self.upmost_output = Some(ty);
          }
        } else {
          if let Some(lowest) = &mut self.lowest_input {
            if analyzer.match_covariant_types(1, ty, *lowest).matched() {
              *lowest = ty;
            }
          } else {
            self.lowest_input = Some(ty);
          }
        }
      }
    }

    let mut inferred = FxHashMap::<SymbolId, InferenceState<'a>>::default();

    fn handle_match_result<'a>(
      analyzer: &mut Analyzer<'a>,
      inferred: &mut FxHashMap<SymbolId, InferenceState<'a>>,
      result: MatchResult<'a>,
    ) -> Option<()> {
      match result {
        MatchResult::Error | MatchResult::Unmatched => return None,
        MatchResult::Matched => {}
        MatchResult::Inferred(i) => {
          for (symbol, (specificity, ty)) in i {
            let state = inferred.entry(symbol).or_default();
            state.update(analyzer, specificity, ty);
          }
        }
      }
      Some(())
    }

    for (arg, (_, param)) in arguments.iter().zip(params.iter()) {
      match arg {
        Argument::SpreadElement(node) => {
          todo!()
        }
        node => {
          let arg = self.exec_expression(node.to_expression(), None);
          let result = self.match_covariant_types(1, arg, *param);
          handle_match_result(self, &mut inferred, result);
        }
      }
    }

    if let Some(ret_sat) = ret_sat {
      let actual_ret = self.resolve_ctx_ty(scope, callable.return_type);
      let result = self.match_covariant_types(1, actual_ret, ret_sat);
      handle_match_result(self, &mut inferred, result);
    }

    for param in &callable.type_params {
      let ty = if let Some(inferred) = inferred.get(&param.symbol_id) {
        inferred.upmost_output.or(inferred.lowest_input)?
      } else {
        // Or constraint? idk
        Ty::Unknown
      };
      self.type_scopes.insert_on_scope(scope, param.symbol_id, ty);
    }

    Some(self.resolve_ctx_ty(scope, callable.return_type))
  }
}
