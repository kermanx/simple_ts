use oxc::{
  ast::ast::{TSTupleElement, TSType},
  span::{Atom, SPAN},
};

use crate::Analyzer;

use super::{Ty, property_key::PropertyKeyType};

#[derive(Debug)]
pub struct TupleElement<'a> {
  pub name: Option<&'a Atom<'a>>,
  pub spread: bool,
  pub optional: bool,
  pub ty: Ty<'a>,
}

#[derive(Debug)]
pub struct TupleType<'a> {
  pub elements: &'a [TupleElement<'a>],
  pub readonly: bool,
}

impl<'a> TupleType<'a> {
  pub fn iterate_result_union(&self, analyzer: &mut Analyzer<'a>) -> Ty<'a> {
    let mut types = Vec::new();

    for element in self.elements {
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
        if let Ok(index) = s.parse::<usize>() {
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
    for (i, element) in self.elements.iter().enumerate() {
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
  pub fn serialize_tuple_type(&mut self, tuple: &TupleType<'a>) -> TSType<'a> {
    let mut elements = self.ast_builder.vec();
    for element in tuple.elements {
      let ty = self.serialize_type(element.ty);
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
          self.ast_builder.identifier_name(SPAN, *name),
          node,
          element.optional,
        ));
      }
      elements.push(node);
    }
    self.ast_builder.ts_type_tuple_type(SPAN, elements)
  }
}
