use crate::{analyzer::Analyzer, ast::DeclarationKind, entity::Entity};
use oxc::ast::ast::{VariableDeclaration, VariableDeclarationKind};

impl<'a> Analyzer<'a> {
  pub fn declare_variable_declaration(
    &mut self,
    node: &'a VariableDeclaration<'a>,
    exporting: bool,
  ) {
    let kind = match &node.kind {
      VariableDeclarationKind::Var => DeclarationKind::Var,
      VariableDeclarationKind::Let => DeclarationKind::Let,
      VariableDeclarationKind::Const => DeclarationKind::Const,
      _ => unimplemented!("using statement"),
    };

    for declarator in &node.declarations {
      self.declare_variable_declarator(declarator, exporting, kind);
    }
  }

  pub fn init_variable_declaration(
    &mut self,
    node: &'a VariableDeclaration<'a>,
    init: Option<Entity<'a>>,
  ) {
    if init.is_some() {
      assert_eq!(node.declarations.len(), 1);
    }

    for declarator in &node.declarations {
      self.init_variable_declarator(declarator, init);
    }
  }
}
