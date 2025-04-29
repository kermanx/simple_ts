use oxc::ast::ast::{TSModuleDeclaration, TSModuleDeclarationBody, TSModuleDeclarationName};

use crate::{Analyzer, ty::namespace::Ns};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_module(&mut self, node: &'a TSModuleDeclaration<'a>) {
    match &node.body {
      Some(TSModuleDeclarationBody::TSModuleBlock(block)) => {
        let ns = self.allocator.alloc(Ns::new_in(self.allocator));

        if let TSModuleDeclarationName::Identifier(id) = &node.id {
          self.namespaces.insert(id.symbol_id(), ns);
        } else {
          // ERROR
        }

        self.active_namespaces.push(ns);
        if node.declare {
          self.declare_scopes += 1;
        }

        for stmt in &block.body {
          self.declare_statement(stmt);
        }

        self.active_namespaces.pop();
        if node.declare {
          self.declare_scopes -= 1;
        }
      }
      _ => todo!(),
    }
  }

  pub fn init_ts_module(&mut self, node: &'a TSModuleDeclaration<'a>) {
    match &node.body {
      Some(TSModuleDeclarationBody::TSModuleBlock(block)) => {
        for stmt in &block.body {
          self.init_statement(stmt);
        }
      }
      _ => todo!(),
    }
  }
}
