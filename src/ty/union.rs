use super::{property_key::PropertyKeyType, unresolved::UnresolvedType, Ty};
use crate::{analyzer::Analyzer, utils::F64WithEq};
use oxc::{
  allocator::Allocator,
  ast::ast::TSType,
  semantic::SymbolId,
  span::{Atom, SPAN},
};
use rustc_hash::FxHashSet;
use std::{hash::Hash, mem};

#[derive(Debug, Default, Clone)]
pub enum UnionType<'a> {
  #[default]
  Never,
  Error,
  Any,
  Unknown,
  Compound(Box<CompoundUnion<'a>>),
  WithUnresolved(Box<UnionType<'a>>, Vec<UnresolvedType<'a>>),
}

impl<'a> UnionType<'a> {
  pub fn add(&mut self, ty: Ty<'a>) {
    match (self, ty) {
      (UnionType::Error | UnionType::Any | UnionType::Unknown, _) => {}
      (s, Ty::Error) => *s = UnionType::Error,
      (s, Ty::Any) => *s = UnionType::Any,
      (s, Ty::Unknown) => *s = UnionType::Unknown,
      (_, Ty::Never) => {}

      (UnionType::WithUnresolved(_, t), Ty::Unresolved(u)) => t.push(u),
      (UnionType::WithUnresolved(s, _), ty) => {
        s.add(ty);
      }
      (s, Ty::Unresolved(u)) => *s = UnionType::WithUnresolved(Box::new(mem::take(s)), vec![u]),

      (s, Ty::Union(tys)) => {
        tys.for_each(|ty| s.add(ty));
      }

      // The rest should be added to compound
      (s @ UnionType::Never, compound) => {
        *s = UnionType::Compound(Box::new(CompoundUnion::default()));
        s.add(compound);
      }
      (UnionType::Compound(c), compound) => {
        c.add(compound);
      }
    }
  }

  pub fn for_each(&self, mut f: impl FnMut(Ty<'a>) -> ()) {
    match self {
      UnionType::Never => f(Ty::Never),
      UnionType::Error => f(Ty::Error),
      UnionType::Any => f(Ty::Any),
      UnionType::Unknown => f(Ty::Unknown),

      UnionType::Compound(c) => {
        c.for_each(f);
      }

      UnionType::WithUnresolved(s, t) => {
        // FIXME: This is ugly. But `&mut f` will trigger a rustc error
        s.for_each(&mut f as &mut dyn FnMut(Ty<'a>) -> ());
        t.iter().copied().for_each(|t| f(Ty::Unresolved(t)));
      }
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct CompoundUnion<'a> {
  string: LiteralAble<&'a Atom<'a>>,
  number: LiteralAble<F64WithEq>,
  bigint: LiteralAble<&'a Atom<'a>>,
  symbol: LiteralAble<SymbolId>,

  object: bool,
  void: bool,
  null: bool,
  undefined: bool,
  /// (has_true, has_false)
  boolean: (bool, bool),

  /// Must be ordered
  /// TODO: Use a set
  complex: Vec<Ty<'a>>,
}

impl<'a> CompoundUnion<'a> {
  pub fn add(&mut self, ty: Ty<'a>) {
    match ty {
      Ty::Error | Ty::Any | Ty::Unknown | Ty::Never | Ty::Union(_) | Ty::Unresolved(_) => {
        unreachable!("Handled in UnionType")
      }

      Ty::Void => self.void = true,
      Ty::Null => self.null = true,
      Ty::Undefined => self.undefined = true,
      Ty::Object => self.object = true,

      Ty::String => self.string = LiteralAble::Any,
      Ty::Number => self.number = LiteralAble::Any,
      Ty::BigInt => self.bigint = LiteralAble::Any,
      Ty::Symbol => self.symbol = LiteralAble::Any,
      Ty::Boolean => self.boolean = (true, true),

      Ty::StringLiteral(s) => self.string.add(s),
      Ty::NumericLiteral(n) => self.number.add(n),
      Ty::BigIntLiteral(b) => self.bigint.add(b),
      Ty::UniqueSymbol(s) => self.symbol.add(s),
      Ty::BooleanLiteral(true) => self.boolean.0 = true,
      Ty::BooleanLiteral(false) => self.boolean.1 = true,

      Ty::Record(_)
      | Ty::Function(_)
      | Ty::Constructor(_)
      | Ty::Namespace(_)
      | Ty::Intersection(_) => {
        self.complex.push(ty);
      }

      Ty::Generic(_) | Ty::Intrinsic(_) => unreachable!("Non-value"),
    }
  }

  pub fn for_each(&self, mut f: impl FnMut(Ty<'a>) -> ()) {
    self.string.for_each(Ty::String, Ty::StringLiteral, &mut f);
    self.number.for_each(Ty::Number, Ty::NumericLiteral, &mut f);
    self.bigint.for_each(Ty::BigInt, Ty::BigIntLiteral, &mut f);
    self.symbol.for_each(Ty::Symbol, Ty::UniqueSymbol, &mut f);

    if self.object {
      f(Ty::Object);
    }
    if self.void {
      f(Ty::Void);
    }
    if self.null {
      f(Ty::Null);
    }
    if self.undefined {
      f(Ty::Undefined);
    }
    match (self.boolean.0, self.boolean.1) {
      (true, true) => f(Ty::Boolean),
      (true, false) => f(Ty::BooleanLiteral(true)),
      (false, true) => f(Ty::BooleanLiteral(false)),
      (false, false) => {}
    }

    self.complex.iter().copied().for_each(f);
  }
}

#[derive(Debug, Default, Clone)]
pub enum LiteralAble<L> {
  #[default]
  Vacant,
  Any,
  Literals(FxHashSet<L>),
}

impl<'a, L> LiteralAble<L> {
  pub fn add(&mut self, literal: L)
  where
    L: Hash + Eq,
  {
    match self {
      LiteralAble::Vacant => {
        *self = LiteralAble::Literals({
          let mut set = FxHashSet::default();
          set.insert(literal);
          set
        })
      }
      LiteralAble::Any => {}
      LiteralAble::Literals(set) => {
        set.insert(literal);
      }
    }
  }

  pub fn for_each(&self, any: Ty<'a>, ctor: fn(L) -> Ty<'a>, mut f: impl FnMut(Ty<'a>) -> ())
  where
    L: Copy,
  {
    match self {
      LiteralAble::Vacant => {}
      LiteralAble::Any => f(any),
      LiteralAble::Literals(set) => {
        set.iter().copied().map(ctor).for_each(&mut f as &mut dyn FnMut(Ty<'a>) -> ())
      }
    }
  }
}

pub fn into_union<'a, Iter>(
  allocator: &'a Allocator,
  types: impl IntoIterator<Item = Ty<'a>, IntoIter = Iter>,
) -> Ty<'a>
where
  Iter: Iterator<Item = Ty<'a>> + ExactSizeIterator,
{
  let mut iter = types.into_iter();
  match iter.len() {
    // FIXME: Should be Ty::Never
    0 => Ty::Undefined,
    1 => iter.next().unwrap(),
    _ => Ty::Union({
      let union = allocator.alloc(UnionType::default());
      iter.for_each(|ty| union.add(ty));
      union
    }),
  }
}

impl<'a> Analyzer<'a> {
  pub fn get_optional_type(&mut self, optional: bool, ty: Ty<'a>) -> Ty<'a> {
    if optional {
      into_union(self.allocator, [Ty::Undefined, ty])
    } else {
      ty
    }
  }

  pub fn get_union_property(&mut self, union: &UnionType<'a>, key: PropertyKeyType<'a>) -> Ty<'a> {
    let result = self.allocator.alloc(UnionType::default());
    union.for_each(|ty| result.add(self.get_property(ty, key)));
    Ty::Union(result)
  }

  pub fn print_union_type(&self, union: &UnionType<'a>) -> TSType<'a> {
    let mut types = self.ast_builder.vec();
    union.for_each(|ty| types.push(self.print_type(ty)));
    self.ast_builder.ts_type_union_type(SPAN, types)
  }
}
