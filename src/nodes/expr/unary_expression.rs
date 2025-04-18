use oxc::{
  ast::ast::{Expression, UnaryExpression, UnaryOperator},
  span::Atom,
};

use crate::{
  analyzer::Analyzer,
  ty::{Ty, facts::Facts, property_key::PropertyKeyType},
};

impl<'a> Analyzer<'a> {
  pub fn exec_unary_expression(
    &mut self,
    node: &'a UnaryExpression,
    _sat: Option<Ty<'a>>,
  ) -> Ty<'a> {
    if node.operator == UnaryOperator::Delete {
      match &node.argument {
        Expression::StaticMemberExpression(node) => {
          let object = self.exec_expression(&node.object, None);
          let key = PropertyKeyType::StringLiteral(&node.property.name);
          self.delete_property(object, key)
        }
        Expression::PrivateFieldExpression(node) => {
          self.add_diagnostic("SyntaxError: private fields can't be deleted");
          let _object = self.exec_expression(&node.object, None);
        }
        Expression::ComputedMemberExpression(node) => {
          let object = self.exec_expression(&node.object, None);
          let key = self.exec_expression(&node.expression, None);
          let key = self.to_property_key(key);
          self.delete_property(object, key)
        }
        Expression::Identifier(_node) => {
          self.add_diagnostic("SyntaxError: Delete of an unqualified identifier in strict mode");
        }
        expr => {
          self.exec_expression(expr, None);
        }
      };

      return Ty::Boolean;
    }

    let argument = self.exec_expression(&node.argument, None);

    match &node.operator {
      UnaryOperator::UnaryNegation => todo!(),
      UnaryOperator::UnaryPlus => self.get_to_numeric(argument),
      UnaryOperator::LogicalNot => Ty::Boolean,
      UnaryOperator::BitwiseNot => self.get_to_numeric(argument),
      UnaryOperator::Typeof => {
        let facts = self.get_facts(argument);
        let values = TYPEOF_VALUES
          .iter()
          .filter_map(|(fact, value)| facts.contains(*fact).then_some(Ty::StringLiteral(value)))
          .collect::<Vec<_>>();
        self.into_union(values).unwrap()
      }
      UnaryOperator::Void => Ty::Undefined,
      UnaryOperator::Delete => unreachable!(),
    }
  }
}

const TYPEOF_VALUES: [(Facts, Atom<'static>); 8] = [
  (Facts::NE_UNDEFINED, Atom::new_const("undefined")),
  (Facts::T_NE_BIGINT, Atom::new_const("bigint")),
  (Facts::T_NE_BOOLEAN, Atom::new_const("boolean")),
  (Facts::T_NE_FUNCTION, Atom::new_const("function")),
  (Facts::T_NE_NUMBER, Atom::new_const("number")),
  (Facts::T_NE_OBJECT, Atom::new_const("object")),
  (Facts::T_NE_STRING, Atom::new_const("string")),
  (Facts::T_NE_SYMBOL, Atom::new_const("symbol")),
];
