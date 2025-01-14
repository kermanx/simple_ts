use crate::{analyzer::Analyzer, r#type::Type};
use oxc::ast::ast::{BindingPattern, BindingPatternKind};

impl<'a> Analyzer<'a> {
  pub fn declare_binding_pattern(&mut self, node: &'a BindingPattern<'a>, mut typed: bool) {
    typed |= node.type_annotation.is_some();
    match &node.kind {
      BindingPatternKind::BindingIdentifier(node) => {
        self.declare_binding_identifier(node, typed);
      }
      BindingPatternKind::ObjectPattern(node) => {
        for property in &node.properties {
          self.declare_binding_pattern(&property.value, typed);
        }
        if let Some(rest) = &node.rest {
          self.declare_binding_rest_element(rest, typed);
        }
      }
      BindingPatternKind::ArrayPattern(node) => {
        for element in node.elements.iter().flatten() {
          self.declare_binding_pattern(element, typed);
        }
        if let Some(rest) = &node.rest {
          self.declare_binding_rest_element(rest, typed);
        }
      }
      BindingPatternKind::AssignmentPattern(node) => {
        self.declare_binding_pattern(&node.left, typed);
      }
    }
  }

  pub fn init_binding_pattern(&mut self, node: &'a BindingPattern<'a>, mut init: Option<Type<'a>>) {
    if let Some(annotation) = &node.type_annotation {
      init = Some(self.resolve_type_annotation(annotation));
    }
    match &node.kind {
      BindingPatternKind::BindingIdentifier(node) => {
        self.init_binding_identifier(node, init);
      }
      BindingPatternKind::ObjectPattern(node) => {
        let init = init.unwrap_or(Type::Undefined);

        let mut enumerated = vec![];
        for property in &node.properties {
          let key = self.exec_property_key(&property.key);

          enumerated.push(key);
          let init = self.get_property(init, key);
          self.init_binding_pattern(&property.value, Some(init));
        }
        if let Some(rest) = &node.rest {
          let init = self.exec_object_rest(init, enumerated);
          self.init_binding_rest_element(rest, init);
        }
      }
      BindingPatternKind::ArrayPattern(node) => {
        let init = init.unwrap_or(Type::Undefined);

        let (element_values, rest_value) =
          self.destruct_as_array(init, node.elements.len(), node.rest.is_some());

        for (element, value) in node.elements.iter().zip(element_values) {
          if let Some(element) = element {
            self.init_binding_pattern(element, Some(value));
          }
        }
        if let Some(rest) = &node.rest {
          self.init_binding_rest_element(rest, rest_value.unwrap());
        }
      }
      BindingPatternKind::AssignmentPattern(node) => {
        let binding_val = self.exec_with_default(&node.right, init.unwrap());

        self.init_binding_pattern(&node.left, Some(binding_val));
      }
    }
  }
}
