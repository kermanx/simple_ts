use crate::{
  analyzer::Analyzer,
  r#type::{facts::Facts, union::into_union, Type},
};
use oxc::{
  ast::ast::{Expression, UnaryExpression, UnaryOperator},
  span::Atom,
};

impl<'a> Analyzer<'a> {
  pub fn exec_unary_expression(&mut self, node: &'a UnaryExpression) -> Type<'a> {
    if node.operator == UnaryOperator::Delete {
      match &node.argument {
        Expression::StaticMemberExpression(node) => {
          let object = self.exec_expression(&node.object);
          let property = Type::StringLiteral(&node.property.name);
          self.delete_property(object, property)
        }
        Expression::PrivateFieldExpression(node) => {
          self.add_diagnostic("SyntaxError: private fields can't be deleted");
          let _object = self.exec_expression(&node.object);
        }
        Expression::ComputedMemberExpression(node) => {
          let object = self.exec_expression(&node.object);
          let property = self.exec_expression(&node.expression);
          self.delete_property(object, property)
        }
        Expression::Identifier(_node) => {
          self.add_diagnostic("SyntaxError: Delete of an unqualified identifier in strict mode");
        }
        expr => {
          self.exec_expression(expr);
        }
      };

      return Type::Boolean;
    }

    let argument = self.exec_expression(&node.argument);

    match &node.operator {
      UnaryOperator::UnaryNegation => todo!(),
      UnaryOperator::UnaryPlus => self.get_to_numeric(argument),
      UnaryOperator::LogicalNot => Type::Boolean,
      UnaryOperator::BitwiseNot => self.get_to_numeric(argument),
      UnaryOperator::Typeof => {
        let facts = self.get_facts(argument);
        let allocator = self.allocator;
        let values = TYPEOF_VALUES
          .iter()
          .filter_map(|(fact, value)| {
            // FIXME: use static atom after next oxc release. See https://github.com/oxc-project/oxc/pull/8479
            facts.contains(*fact).then(|| Type::StringLiteral(allocator.alloc(Atom::from(*value))))
          })
          .collect::<Vec<_>>();
        into_union(self.allocator, values)
      }
      UnaryOperator::Void => Type::Undefined,
      UnaryOperator::Delete => unreachable!(),
    }
  }
}

const TYPEOF_VALUES: [(Facts, &'static str); 8] = [
  (Facts::NE_UNDEFINED, "undefined"),
  (Facts::T_NE_BIGINT, "bigint"),
  (Facts::T_NE_BOOLEAN, "boolean"),
  (Facts::T_NE_FUNCTION, "function"),
  (Facts::T_NE_NUMBER, "number"),
  (Facts::T_NE_OBJECT, "object"),
  (Facts::T_NE_STRING, "string"),
  (Facts::T_NE_SYMBOL, "symbol"),
];
