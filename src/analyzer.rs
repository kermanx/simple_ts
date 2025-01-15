use crate::{
  config::Config,
  scope::{
    call::CallScope,
    cf::{CfScope, CfScopeKind},
    tree::ScopeTree,
    variable::VariableScope,
  },
  ty::Ty,
};
use line_index::LineIndex;
use oxc::{
  allocator::Allocator,
  ast::ast::Program,
  semantic::{Semantic, SymbolId},
  span::{GetSpan, Span},
};
use rustc_hash::FxHashMap;
use std::collections::BTreeSet;

pub struct Analyzer<'a> {
  pub allocator: &'a Allocator,
  pub config: &'a Config,
  pub line_index: LineIndex,
  pub semantic: Semantic<'a>,
  pub diagnostics: BTreeSet<String>,

  pub span_stack: Vec<Span>,
  pub call_scopes: Vec<CallScope<'a>>,
  pub cf_scopes: ScopeTree<CfScope<'a>>,
  pub variable_scopes: ScopeTree<VariableScope<'a>>,

  pub variables: FxHashMap<SymbolId, Ty<'a>>,
  pub types: FxHashMap<SymbolId, Ty<'a>>,
}

impl<'a> Analyzer<'a> {
  pub fn new(allocator: &'a Allocator, config: Config, semantic: Semantic<'a>) -> Self {
    let config = allocator.alloc(config);

    let mut cf_scopes = ScopeTree::new();
    let root_cf_scope = cf_scopes.push(CfScope { kind: CfScopeKind::Module, exited: None });

    let mut variable_scopes = ScopeTree::new();
    let root_variable_scope = variable_scopes.push(VariableScope::new(root_cf_scope));

    let root_call_scope =
      CallScope::new(vec![], root_variable_scope, 0, true, false, todo!("globalThis"));

    Analyzer {
      config,
      line_index: LineIndex::new(semantic.source_text()),
      semantic,
      diagnostics: Default::default(),

      span_stack: Vec::new(),
      call_scopes: Vec::from([root_call_scope]),
      cf_scopes,
      variable_scopes,

      variables: Default::default(),
      types: Default::default(),

      allocator,
    }
  }

  pub fn exec_program(&mut self, node: &'a Program<'a>) {
    self.exec_statement_vec(&node.body);

    assert_eq!(self.variable_scopes.stack.len(), 1);

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

  pub fn push_span(&mut self, node: &'a impl GetSpan) {
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
}
