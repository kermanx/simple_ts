use super::{Entity, EnumeratedProperties, IteratedElements};
use crate::analyzer::Analyzer;

pub fn get_property<'a>(
  _target: Entity<'a>,
  analyzer: &mut Analyzer<'a>,
  _key: Entity<'a>,
) -> Entity<'a> {
  analyzer.factory.unknown
}

pub fn set_property<'a>(analyzer: &mut Analyzer<'a>, _key: Entity<'a>, value: Entity<'a>) {
  value.unknown_mutation(analyzer);
}

pub fn enumerate_properties<'a>(
  _target: Entity<'a>,
  analyzer: &mut Analyzer<'a>,
) -> EnumeratedProperties<'a> {
  vec![(false, analyzer.factory.unknown_primitive, analyzer.factory.unknown)]
}

pub fn delete_property<'a>(_analyzer: &mut Analyzer<'a>, _key: Entity<'a>) {}

pub fn call<'a>(
  _target: Entity<'a>,
  analyzer: &mut Analyzer<'a>,

  this: Entity<'a>,
  args: Entity<'a>,
) -> Entity<'a> {
  this.unknown_mutation(analyzer);
  args.unknown_mutation(analyzer);
  analyzer.factory.unknown
}

pub fn construct<'a>(
  _target: Entity<'a>,
  analyzer: &mut Analyzer<'a>,

  args: Entity<'a>,
) -> Entity<'a> {
  args.unknown_mutation(analyzer);
  analyzer.factory.unknown
}

pub fn jsx<'a>(_target: Entity<'a>, analyzer: &mut Analyzer<'a>, _props: Entity<'a>) -> Entity<'a> {
  // No consume!
  analyzer.factory.unknown
}

pub fn r#await<'a>(analyzer: &mut Analyzer<'a>) -> Entity<'a> {
  analyzer.factory.unknown
}

pub fn iterate<'a>(analyzer: &mut Analyzer<'a>) -> IteratedElements<'a> {
  (vec![], Some(analyzer.factory.unknown))
}

pub fn get_to_string<'a>(analyzer: &Analyzer<'a>) -> Entity<'a> {
  analyzer.factory.unknown_string
}

pub fn get_to_numeric<'a>(analyzer: &Analyzer<'a>) -> Entity<'a> {
  // Possibly number or bigint
  analyzer.factory.unknown
}
