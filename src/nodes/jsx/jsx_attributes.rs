use crate::{analyzer::Analyzer, ty::record::Record};
use oxc::ast::ast::{JSXAttributeItem, JSXOpeningElement};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attributes(&mut self, node: &'a JSXOpeningElement<'a>) -> &'a mut Record<'a> {
    let object = self.allocator.alloc(Record::default());

    for attr in &node.attributes {
      match attr {
        JSXAttributeItem::Attribute(node) => {
          let key = self.exec_jsx_attribute_name(&node.name);
          let value = self.exec_jsx_attribute_value(&node.value);
          object.init_property(self, key, value, false, false);
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
