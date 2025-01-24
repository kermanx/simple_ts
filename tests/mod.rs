use std::{fs, sync::LazyLock};

use insta::{assert_snapshot, glob, Settings};
use oxc::{
  allocator::Allocator,
  ast::{ast::Statement, NONE},
  codegen::Codegen,
  span::{SourceType, SPAN},
};
use regex::Regex;
use simple_ts::{analyze, Config};

static TYPE_QUERY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\^\? (\w+)").unwrap());

pub fn serialize_queried_types(code: String) -> String {
  let allocator = Allocator::default();
  let code = allocator.alloc(code);
  let mut analyzer = analyze(&allocator, code, Config::default());
  let codegen = Codegen::new();

  let mut snapshot_stmts = analyzer.ast_builder.vec();
  for query in TYPE_QUERY_RE.find_iter(code) {
    let name = &query.as_str()[3..];
    let mark_offset = query.start();
    let mut line_col = analyzer.line_index.line_col(mark_offset.try_into().unwrap());
    line_col.line -= 1;
    let start_offset = analyzer.line_index.offset(line_col).unwrap();
    let ty = analyzer.get_type_by_pos(start_offset.into()).unwrap_or_else(|| {
      panic!("Type query `{}` at {}:{} not found", name, line_col.line + 1, line_col.col + 1)
    });
    snapshot_stmts.push(Statement::from(analyzer.ast_builder.declaration_ts_type_alias(
      SPAN,
      analyzer.ast_builder.binding_identifier(SPAN, name),
      NONE,
      analyzer.serialize_type(ty),
      false,
    )));
  }

  codegen
    .build(&analyzer.ast_builder.program(
      SPAN,
      SourceType::tsx(),
      "",
      analyzer.ast_builder.vec(),
      None,
      analyzer.ast_builder.vec(),
      snapshot_stmts,
    ))
    .code
}

#[test]
fn test() {
  glob!("fixtures/**/*.ts", |path| {
    let input = fs::read_to_string(path).unwrap();
    let mut settings = Settings::clone_current();
    settings.set_omit_expression(true);
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
      assert_snapshot!(serialize_queried_types(input));
    })
  });
}
