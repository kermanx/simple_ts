mod allocator;
mod analyzer;
mod builtins;
mod config;
mod nodes;
mod scope;
pub mod ty;
mod utils;

pub use analyzer::Analyzer;
pub use config::Config;
use oxc::{allocator::Allocator, parser::Parser, semantic::SemanticBuilder, span::SourceType};

pub fn analyze<'a>(allocator: &'a Allocator, code: &'a str, config: Config) -> Analyzer<'a> {
  let parsed = Parser::new(allocator, code, SourceType::tsx()).parse();
  let program = allocator.alloc(parsed.program);
  let semantic = SemanticBuilder::new().build(program);
  let mut analyzer = Analyzer::new(allocator, config, semantic.semantic);
  analyzer.exec_program(program);
  analyzer
}
