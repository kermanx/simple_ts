use crate::{analyzer::Analyzer, ty::ObjectEntity};
use oxc::ast::ast::{JSXAttributeItem, JSXOpeningElement, PropertyKind};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attributes(
    &mut self,
    node: &'a JSXOpeningElement<'a>,
  ) -> &'a mut ObjectEntity<'a> {
    let object = self.new_empty_object(&self.builtins.prototypes.object);

    for attr in &node.attributes {
      match attr {
        JSXAttributeItem::Attribute(node) => {
          let key = self.exec_jsx_attribute_name(&node.name);
          let value = self.exec_jsx_attribute_value(&node.value);
          object.init_property(self, PropertyKind::Init, key, value, true);
        }
        JSXAttributeItem::SpreadAttribute(node) => {
          let argument = self.exec_expression(&node.argument);
          object.init_spread(self, argument);
        }
      }
    }

    object
  }
}
