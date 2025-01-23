use oxc::{
  ast::ast::{TSTupleElement, TSType},
  span::{Atom, SPAN},
};

use crate::Analyzer;

use super::{property_key::PropertyKeyType, Ty};

#[derive(Debug)]
pub struct TupleElement<'a> {
  pub name: Option<&'a Atom<'a>>,
  pub spread: bool,
  pub optional: bool,
  pub ty: Ty<'a>,
}

#[derive(Debug, Default)]
pub struct TupleType<'a>(pub Vec<TupleElement<'a>>);

impl<'a> TupleType<'a> {
  pub fn iterate_result_union(&self, analyzer: &mut Analyzer<'a>) -> Ty<'a> {
    let mut types = Vec::new();

    for element in &self.0 {
      if element.spread {
        types.push(analyzer.iterate_result_union(element.ty));
      } else {
        types.push(element.ty);
      }
    }

    analyzer.into_union(types).unwrap_or(Ty::Never)
  }

  pub fn get_property(&self, key: PropertyKeyType<'a>, analyzer: &mut Analyzer<'a>) -> Ty<'a> {
    match key {
      PropertyKeyType::Error => Ty::Error,
      PropertyKeyType::AnyString => Ty::Error,
      PropertyKeyType::AnyNumber => self.iterate_result_union(analyzer),
      PropertyKeyType::AnySymbol => Ty::Error,
      PropertyKeyType::StringLiteral(s) => {
        if let Some(index) = s.parse::<usize>().ok() {
          self.get_element_by_index(index, analyzer)
        } else {
          todo!("Array prototype");
        }
      }
      PropertyKeyType::NumericLiteral(n) => {
        let index = n.0 as usize;
        self.get_element_by_index(index, analyzer)
      }
      PropertyKeyType::UniqueSymbol(s) => {
        todo!("Array prototype");
      }
    }
    // self.0.get(index).map(|e| e.ty)
  }

  fn get_element_by_index(&self, index: usize, analyzer: &mut Analyzer<'a>) -> Ty<'a> {
    let mut determinate = true;
    let mut types = Vec::new();
    for (i, element) in self.0.iter().enumerate() {
      if element.spread {
        determinate = false;
      }
      if determinate {
        if i == index {
          return element.ty;
        }
      } else {
        types.push(if element.spread {
          analyzer.iterate_result_union(element.ty)
        } else {
          element.ty
        });
      }
    }
    analyzer.into_union(types).unwrap_or(Ty::Undefined)
  }
}

impl<'a> Analyzer<'a> {
  pub fn print_tuple_type(&self, tuple: &TupleType<'a>) -> TSType<'a> {
    let mut elements = self.ast_builder.vec();
    for element in &tuple.0 {
      let ty = self.print_type(element.ty);
      let mut node = if element.optional && element.name.is_none() {
        self.ast_builder.ts_tuple_element_optional_type(SPAN, ty)
      } else if element.spread {
        self.ast_builder.ts_tuple_element_rest_type(SPAN, ty)
      } else {
        TSTupleElement::from(ty)
      };
      if let Some(name) = element.name {
        node = TSTupleElement::from(self.ast_builder.ts_type_named_tuple_member(
          SPAN,
          node,
          self.ast_builder.identifier_name(SPAN, name),
          element.optional,
        ));
      }
      elements.push(node);
    }
    self.ast_builder.ts_type_tuple_type(SPAN, elements)
  }
}
