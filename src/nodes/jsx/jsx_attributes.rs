use oxc::ast::ast::{JSXAttributeItem, JSXOpeningElement};

use crate::{
  analyzer::Analyzer,
  ty::{
    record::{RecordType, RecordTypeBuilder},
    Ty,
  },
};

impl<'a> Analyzer<'a> {
  pub fn exec_jsx_attributes(
    &mut self,
    node: &'a JSXOpeningElement<'a>,
    sat: Option<Ty<'a>>,
  ) -> &'a mut RecordType<'a> {
    let mut object = RecordTypeBuilder::default();

    for attr in &node.attributes {
      match attr {
        JSXAttributeItem::Attribute(node) => {
          let key = self.exec_jsx_attribute_name(&node.name);
          let sat = sat.map(|sat| self.get_property(sat, key));
          let value = self.exec_jsx_attribute_value(&node.value, sat);
          object.init_property(self, key, value, false, false);
        }
        JSXAttributeItem::SpreadAttribute(node) => {
          let argument = self.exec_expression(&node.argument, None);
          object.init_spread(self, argument);
        }
      }
    }

    self.allocator.alloc(object.build())
  }
}
