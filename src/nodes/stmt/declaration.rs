use crate::analyzer::Analyzer;
use oxc::ast::ast::Declaration;

impl<'a> Analyzer<'a> {
  pub fn declare_declaration(&mut self, node: &'a Declaration<'a>) {
    match node {
      Declaration::VariableDeclaration(node) => {
        self.declare_variable_declaration(node);
      }
      Declaration::FunctionDeclaration(node) => {
        self.declare_function(node);
      }
      Declaration::ClassDeclaration(node) => {
        self.declare_class(node);
      }

      Declaration::TSTypeAliasDeclaration(node) => {
        self.declare_ts_type_alias(node);
      }
      _ => todo!(),
    }
  }

  pub fn init_declaration(&mut self, node: &'a Declaration<'a>) {
    match node {
      Declaration::VariableDeclaration(node) => {
        self.init_variable_declaration(node, None);
      }
      Declaration::FunctionDeclaration(_node) => {
        // Nothing to do
      }
      Declaration::ClassDeclaration(node) => {
        self.init_class(node);
      }

      Declaration::TSTypeAliasDeclaration(node) => {
        self.init_ts_type_alias(node);
      }
      _ => todo!(),
    }
  }
}
