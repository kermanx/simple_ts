mod analyzer;
mod builtins;
mod config;
mod nodes;
mod scope;
mod ty;
mod utils;

use analyzer::Analyzer;
use config::Config;
use oxc::{allocator::Allocator, parser::Parser, semantic::SemanticBuilder, span::SourceType};

pub fn analyze(code: String, config: Config) {
  let allocator = Allocator::default();
  let parsed = Parser::new(&allocator, code.as_str(), SourceType::tsx()).parse();
  let semantic = SemanticBuilder::new().build(&parsed.program);
  let mut analyzer = Analyzer::new(&allocator, config, semantic.semantic);
  analyzer.exec_program(&parsed.program);
}
