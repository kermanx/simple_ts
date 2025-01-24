use std::collections::BTreeSet;

use line_index::LineIndex;
use oxc::{
  allocator::Allocator,
  ast::{ast::Program, AstBuilder},
  semantic::{Semantic, SymbolId},
  span::{GetSpan, Span, SPAN},
};
use rustc_hash::FxHashMap;

use crate::{
  builtins::Builtins,
  config::Config,
  scope::{
    call::CallScope,
    control::CfScopeKind,
    r#type::TypeScopeTree,
    runtime::{RuntimeScope, RuntimeScopeTree},
  },
  ty::{accumulator::TypeAccumulator, ctx::CtxTy, Ty},
};

pub struct Analyzer<'a> {
  pub allocator: &'a Allocator,
  pub config: &'a Config,
  pub line_index: LineIndex,
  pub semantic: Semantic<'a>,
  pub ast_builder: AstBuilder<'a>,

  pub builtins: Builtins<'a>,

  pub span_stack: Vec<Span>,
  pub call_scopes: Vec<CallScope<'a>>,
  pub runtime_scopes: RuntimeScopeTree<'a>,
  pub type_scopes: TypeScopeTree<'a>,

  /// Variables with a unique type
  pub variables: FxHashMap<SymbolId, Ty<'a>>,
  /// Generic parameter with its constraint
  pub generic_constraints: FxHashMap<SymbolId, CtxTy<'a>>,

  pub diagnostics: BTreeSet<String>,
  pub span_to_type: FxHashMap<Span, TypeAccumulator<'a>>,
  pub pos_to_span: &'a mut [Span],
}

impl<'a> Analyzer<'a> {
  pub fn new(allocator: &'a Allocator, config: Config, semantic: Semantic<'a>) -> Self {
    let config = allocator.alloc(config);

    let mut runtime_scopes = RuntimeScopeTree::default();
    let root_scope = runtime_scopes.push(RuntimeScope {
      kind: CfScopeKind::Module,
      exited: None,
      variables: Default::default(),
    });

    let root_call_scope =
      CallScope::new(root_scope, true, false, /* TODO: globalThis */ Ty::Any, None);

    let ast_builder = AstBuilder::new(allocator);
    let pos_to_expr = allocator.alloc_slice_fill_default(semantic.source_text().len());

    Analyzer {
      allocator,
      config,
      line_index: LineIndex::new(semantic.source_text()),
      semantic,
      ast_builder,

      builtins: Builtins::new(),

      span_stack: Vec::new(),
      call_scopes: Vec::from([root_call_scope]),
      runtime_scopes,
      type_scopes: TypeScopeTree::new(),

      variables: Default::default(),
      generic_constraints: Default::default(),

      diagnostics: Default::default(),
      span_to_type: Default::default(),
      pos_to_span: pos_to_expr,
    }
  }

  pub fn exec_program(&mut self, node: &'a Program<'a>) {
    self.exec_statement_vec(&node.body);

    assert_eq!(self.runtime_scopes.stack.len(), 1);

    #[cfg(feature = "flame")]
    flamescope::dump(&mut std::fs::File::create("flamescope.json").unwrap()).unwrap();
  }
}

impl<'a> Analyzer<'a> {
  pub fn current_span(&self) -> Span {
    *self.span_stack.last().unwrap()
  }

  pub fn add_diagnostic(&mut self, message: impl Into<String>) {
    let span = self.current_span();
    let start = self.line_index.line_col(span.start.into());
    let end = self.line_index.line_col(span.end.into());
    let span_text =
      format!(" at {}:{}-{}:{}", start.line + 1, start.col + 1, end.line + 1, end.col + 1);
    self.diagnostics.insert(message.into() + &span_text);
  }

  pub fn push_span(&mut self, node: &impl GetSpan) {
    self.span_stack.push(node.span());
  }

  pub fn pop_span(&mut self) {
    self.span_stack.pop();
  }

  pub fn resolve_module(&mut self, specifier: &'a str) -> Option<()> {
    todo!()
  }

  pub fn resolve_global_variable(&mut self, id: &'a str) -> Ty<'a> {
    todo!()
  }

  pub fn resolve_global_type(&mut self, id: &'a str) -> Ty<'a> {
    todo!()
  }

  pub fn accumulate_type(&mut self, span: &impl GetSpan, ty: Ty<'a>) {
    let Analyzer { allocator, span_to_type: expr_types, pos_to_span: pos_to_expr, .. } = self;
    let span = span.span();
    let acc = expr_types.entry(span).or_insert_with(move || {
      for pos in span.start..span.end {
        pos_to_expr[pos as usize] = span;
      }
      TypeAccumulator::default()
    });
    acc.add(ty, allocator);
  }

  pub fn get_type_by_pos(&mut self, pos: usize) -> Option<Ty<'a>> {
    let span = self.pos_to_span[pos];
    if span == SPAN {
      None
    } else {
      self.span_to_type.get_mut(&span).unwrap().to_ty()
    }
  }
}
