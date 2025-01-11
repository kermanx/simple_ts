use crate::{
  ast::AstKind2,
  builtins::Builtins,
  config::Config,
  r#type::{EntityFactory, EntityOpHost, Type},
  scope::{exhaustive::ExhaustiveCallback, ScopeContext},
};
use line_index::LineIndex;
use oxc::{
  allocator::Allocator,
  ast::ast::{LabeledStatement, Program},
  semantic::{Semantic, SymbolId},
  span::{GetSpan, Span},
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::BTreeSet, marker::PhantomData, mem, rc::Rc};

pub struct Analyzer<'a> {
  pub allocator: &'a Allocator,
  pub config: &'a Config,
  pub factory: &'a EntityFactory<'a>,
  pub line_index: LineIndex,
  pub semantic: Semantic<'a>,
  pub span_stack: Vec<Span>,
  pub data: FxHashMap<(usize, usize), Box<PhantomData<&'a ()>>>,
  pub named_exports: Vec<SymbolId>,
  pub default_export: Option<Type<'a>>,
  pub scope_context: ScopeContext<'a>,
  pub pending_labels: Vec<&'a LabeledStatement<'a>>,
  pub pending_deps: FxHashSet<ExhaustiveCallback<'a>>,
  pub builtins: Builtins<'a>,
  pub entity_op: EntityOpHost<'a>,
  pub diagnostics: BTreeSet<String>,
}

impl<'a> Analyzer<'a> {
  pub fn new(allocator: &'a Allocator, config: Config, semantic: Semantic<'a>) -> Self {
    let config = allocator.alloc(config);
    let factory = allocator.alloc(EntityFactory::new(allocator, config));

    Analyzer {
      allocator,
      config,
      factory,
      line_index: LineIndex::new(semantic.source_text()),
      semantic,
      span_stack: vec![],
      data: Default::default(),
      named_exports: Vec::new(),
      default_export: None,
      scope_context: ScopeContext::new(factory),
      pending_labels: Vec::new(),
      pending_deps: Default::default(),
      builtins: Builtins::new(config, factory),
      entity_op: EntityOpHost::new(allocator),
      diagnostics: Default::default(),
    }
  }

  pub fn exec_program(&mut self, node: &'a Program<'a>) {
    self.exec_statement_vec(&node.body);

    self.consume_exports();

    self.scope_context.assert_final_state();

    // println!("debug: {:?}", self.debug);

    #[cfg(feature = "flame")]
    flamescope::dump(&mut std::fs::File::create("flamescope.json").unwrap()).unwrap();
  }

  pub fn consume_exports(&mut self) {
    if let Some(entity) = self.default_export.take() {
      entity.unknown_mutation(self)
    }
    for symbol in self.named_exports.clone() {
      let entity = self.read_symbol(symbol).unwrap();
      entity.unknown_mutation(self);
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn set_data(&mut self, key: AstKind2<'a>, data: impl Default + 'a) {
    self.data.insert(key.into(), unsafe { mem::transmute(Box::new(data)) });
  }

  pub fn get_data_or_insert_with<D: 'a>(
    &mut self,
    key: AstKind2<'a>,
    default: impl FnOnce() -> D,
  ) -> &'a mut D {
    let boxed =
      self.data.entry(key.into()).or_insert_with(|| unsafe { mem::transmute(Box::new(default())) });
    unsafe { mem::transmute(boxed.as_mut()) }
  }

  pub fn load_data<D: Default + 'a>(&mut self, key: AstKind2<'a>) -> &'a mut D {
    self.get_data_or_insert_with(key, Default::default)
  }

  #[allow(clippy::rc_buffer)]
  pub fn take_labels(&mut self) -> Option<Rc<Vec<&'a LabeledStatement<'a>>>> {
    if self.pending_labels.is_empty() {
      None
    } else {
      Some(Rc::new(mem::take(&mut self.pending_labels)))
    }
  }

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
}

impl<'a> From<Analyzer<'a>> for &'a Allocator {
  fn from(val: Analyzer<'a>) -> Self {
    val.allocator
  }
}
