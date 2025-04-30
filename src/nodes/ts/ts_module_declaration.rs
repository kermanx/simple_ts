use oxc::ast::ast::{TSModuleDeclaration, TSModuleDeclarationBody, TSModuleDeclarationName};

use crate::{
  Analyzer,
  ty::{Ty, namespace::Ns},
};

impl<'a> Analyzer<'a> {
  pub fn declare_ts_module(&mut self, node: &'a TSModuleDeclaration<'a>) {
    match &node.body {
      Some(TSModuleDeclarationBody::TSModuleBlock(block)) => {
        let ns = self.allocator.alloc(Ns::new_in(self.allocator));

        if let TSModuleDeclarationName::Identifier(id) = &node.id {
          self.declare_binding_identifier(id, true);
          self.init_binding_identifier(id, Some(Ty::Record(ns.variables())));
          self.declare_namespace_identifier(id, false, ns);
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
        let ns = if let TSModuleDeclarationName::Identifier(id) = &node.id {
          self.namespaces[&id.symbol_id()]
        } else {
          // ERROR
          return;
        };

        self.active_namespaces.push(ns);
        if node.declare {
          self.declare_scopes += 1;
        }

        for stmt in &block.body {
          self.init_statement(stmt);
        }

        self.active_namespaces.pop();
        if node.declare {
          self.declare_scopes -= 1;
        }
      }
      _ => todo!(),
    }
  }
}
