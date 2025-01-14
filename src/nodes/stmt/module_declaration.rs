use crate::{analyzer::Analyzer, ast::DeclarationKind, r#type::Type};
use oxc::ast::ast::{
  ExportDefaultDeclarationKind, ImportDeclarationSpecifier, ModuleDeclaration, ModuleExportName,
};

impl<'a> Analyzer<'a> {
  pub fn declare_module_declaration(&mut self, node: &'a ModuleDeclaration<'a>) {
    match node {
      ModuleDeclaration::ImportDeclaration(node) => {
        if let Some(specifiers) = &node.specifiers {
          let name = node.source.value.as_str();
          let known = self.builtins.get_known_module(name);

          for specifier in specifiers {
            let value = if let Some(known) = known {
              match specifier {
                ImportDeclarationSpecifier::ImportDefaultSpecifier(_node) => known.default,
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_node) => known.namespace,
                ImportDeclarationSpecifier::ImportSpecifier(node) => {
                  let key = Type::StringLiteral(match &node.imported {
                    ModuleExportName::IdentifierName(identifier) => &identifier.name,
                    ModuleExportName::IdentifierReference(identifier) => &identifier.name,
                    ModuleExportName::StringLiteral(literal) => &literal.value,
                  });
                  self.get_property(known.namespace, key)
                }
              }
            } else {
              Type::Unknown
            };

            let local = specifier.local();
            self.declare_binding_identifier(local, false, DeclarationKind::Import);

            self.init_binding_identifier(local, Some(value));
          }
        }
      }
      ModuleDeclaration::ExportNamedDeclaration(node) => {
        if node.source.is_some() {
          // Re-exports. Nothing to do.
          return;
        }
        if let Some(declaration) = &node.declaration {
          self.declare_declaration(declaration, true);
        }
      }
      ModuleDeclaration::ExportDefaultDeclaration(node) => {
        match &node.declaration {
          ExportDefaultDeclarationKind::FunctionDeclaration(node) => {
            if node.id.is_none() {
              // Patch `export default function(){}`
              return;
            }
            // Pass `exporting` as `false` because it is actually used as an expression
            self.declare_function(node, false);
          }
          ExportDefaultDeclarationKind::ClassDeclaration(node) => {
            if node.id.is_none() {
              // Patch `export default class{}`
              return;
            }
            // Pass `exporting` as `false` because it is actually used as an expression
            self.declare_class(node, false);
          }
          _expr => {}
        };
      }
      ModuleDeclaration::ExportAllDeclaration(_node) => {
        // Nothing to do
      }
      _ => unreachable!(),
    }
  }

  pub fn init_module_declaration(&mut self, node: &'a ModuleDeclaration<'a>) {
    match node {
      ModuleDeclaration::ImportDeclaration(_node) => {}
      ModuleDeclaration::ExportNamedDeclaration(node) => {
        if node.source.is_some() {
          // Re-exports. Nothing to do.
          return;
        }
        if let Some(declaration) = &node.declaration {
          self.init_declaration(declaration);
        }
      }
      ModuleDeclaration::ExportDefaultDeclaration(node) => {
        let value = match &node.declaration {
          ExportDefaultDeclarationKind::FunctionDeclaration(node) => self.exec_function(node),
          ExportDefaultDeclarationKind::ClassDeclaration(node) => {
            if node.id.is_none() {
              // Patch `export default class{}`
              self.exec_class(node)
            } else {
              self.init_class(node)
            }
          }
          node => self.exec_expression(node.to_expression()),
        };
      }
      ModuleDeclaration::ExportAllDeclaration(_node) => {
        // Nothing to do
      }
      _ => unreachable!(),
    }
  }
}
