use super::{
  union::into_union,
  unresolved::{UnresolvedIntersection, UnresolvedType},
  Ty,
};
use crate::{analyzer::Analyzer, utils::F64WithEq};
use oxc::{
  allocator::Allocator,
  ast::ast::TSType,
  semantic::SymbolId,
  span::{Atom, SPAN},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum IntersectionBuilderState<'a> {
  #[default]
  Unknown,
  Never,
  Any,
  Error,

  Null,
  Undefined,

  // The followings allow other complex object-like types to intersect with them.
  Number(Option<F64WithEq>),
  String(Option<&'a Atom<'a>>),
  Boolean(Option<bool>),
  Symbol(Option<SymbolId>),
  BigInt(Option<&'a Atom<'a>>),
  ObjectKeyword,
  Void,
  ObjectLike,
}

macro_rules! match_with_literal {
  ($name: ident, $lit: ident) => {
    (Self::$name($lit), Self::ObjectLike)
      | (Self::ObjectLike, Self::$name($lit))
      | (Self::$name(None), Self::$name($lit))
      | (Self::$name($lit), Self::$name(None))
  };
}

impl<'a> IntersectionBuilderState<'a> {
  pub fn intersect(self, other: Self) -> Self {
    match (self, other) {
      (k1, k2) if k1 == k2 => k1,

      (Self::Error, _) | (_, Self::Error) => Self::Error,
      (Self::Never, _) | (_, Self::Never) => Self::Never,
      (Self::Any, _) | (_, Self::Any) => Self::Any,
      (Self::Unknown, k) | (k, Self::Unknown) => k,

      match_with_literal!(Number, lit) => Self::Number(lit),
      match_with_literal!(String, lit) => Self::String(lit),
      match_with_literal!(Boolean, lit) => Self::Boolean(lit),
      match_with_literal!(Symbol, lit) => Self::Symbol(lit),
      match_with_literal!(BigInt, lit) => Self::BigInt(lit),

      (Self::ObjectKeyword, Self::ObjectLike) | (Self::ObjectLike, Self::ObjectKeyword) => {
        Self::ObjectKeyword
      }
      (Self::Void, Self::ObjectLike) | (Self::ObjectLike, Self::Void) => Self::Void,

      (Self::Void, Self::Undefined) | (Self::Undefined, Self::Void) => Self::Undefined,

      _ => Self::Never,
    }
  }
}

#[derive(Debug, Default)]
pub struct IntersectionTypeBuilder<'a> {
  kind: IntersectionBuilderState<'a>,
  object_like: Vec<Ty<'a>>,
  unresolved: Vec<UnresolvedType<'a>>,

  union: Option<Vec<IntersectionTypeBuilder<'a>>>,
}

impl<'a> IntersectionTypeBuilder<'a> {
  pub fn add(&mut self, ty: Ty<'a>) {
    if let Some(union) = &mut self.union {
      for builder in union {
        builder.add(ty);
      }
    } else if self.kind != IntersectionBuilderState::Never {
      let kind = match ty {
        Ty::Error => IntersectionBuilderState::Error,
        Ty::Any => IntersectionBuilderState::Any,
        Ty::Unknown => IntersectionBuilderState::Unknown,
        Ty::Never => IntersectionBuilderState::Never,
        Ty::Void => IntersectionBuilderState::Void,

        Ty::BigInt => IntersectionBuilderState::BigInt(None),
        Ty::Boolean => IntersectionBuilderState::Boolean(None),
        Ty::Null => IntersectionBuilderState::Null,
        Ty::Number => IntersectionBuilderState::Number(None),
        Ty::Object => IntersectionBuilderState::ObjectKeyword,
        Ty::String => IntersectionBuilderState::String(None),
        Ty::Symbol => IntersectionBuilderState::Symbol(None),
        Ty::Undefined => IntersectionBuilderState::Undefined,

        Ty::StringLiteral(s) => IntersectionBuilderState::String(Some(s)),
        Ty::NumericLiteral(n) => IntersectionBuilderState::Number(Some(n)),
        Ty::BigIntLiteral(n) => IntersectionBuilderState::BigInt(Some(n)),
        Ty::BooleanLiteral(b) => IntersectionBuilderState::Boolean(Some(b)),
        Ty::UniqueSymbol(s) => IntersectionBuilderState::Symbol(Some(s)),

        Ty::Record(_)
        | Ty::Function(_)
        | Ty::Constructor(_)
        | Ty::Interface(_)
        | Ty::Namespace(_) => {
          self.object_like.push(ty);
          IntersectionBuilderState::ObjectLike
        }

        Ty::Union(u) => {
          let mut union = Vec::new();
          u.for_each(|ty| {
            let mut builder = IntersectionTypeBuilder::default();
            builder.kind = self.kind;
            builder.add(ty);
            if builder.kind != IntersectionBuilderState::Never {
              union.push(builder);
            }
          });
          if union.is_empty() {
            IntersectionBuilderState::Never
          } else {
            self.union = Some(union);
            return;
          }
        }
        Ty::Intersection(i) => {
          i.for_each(|ty| self.add(ty));
          return;
        }

        Ty::Generic(_) | Ty::Intrinsic(_) => IntersectionBuilderState::Error,

        Ty::Unresolved(u) => {
          self.unresolved.push(u);
          return;
        }
      };
      self.kind = self.kind.intersect(kind);
    }
  }

  fn build_without_union_and_unresolved(
    allocator: &'a Allocator,
    kind: IntersectionBuilderState<'a>,
    object_like: Vec<Ty<'a>>,
  ) -> Ty<'a> {
    let primitive_only = object_like.is_empty();
    let kind = match kind {
      // Ignore complex types
      IntersectionBuilderState::Error => return Ty::Error,
      IntersectionBuilderState::Any => return Ty::Any,
      IntersectionBuilderState::Unknown => return Ty::Unknown,
      IntersectionBuilderState::Never => return Ty::Never,

      IntersectionBuilderState::Null => return Ty::Null,
      IntersectionBuilderState::Undefined => return Ty::Undefined,

      // Primitive only
      IntersectionBuilderState::BigInt(None) if primitive_only => return Ty::BigInt,
      IntersectionBuilderState::BigInt(Some(n)) if primitive_only => return Ty::BigIntLiteral(n),
      IntersectionBuilderState::Boolean(None) if primitive_only => return Ty::Boolean,
      IntersectionBuilderState::Boolean(Some(b)) if primitive_only => return Ty::BooleanLiteral(b),
      IntersectionBuilderState::Number(None) if primitive_only => return Ty::Number,
      IntersectionBuilderState::Number(Some(n)) if primitive_only => return Ty::NumericLiteral(n),
      IntersectionBuilderState::String(None) if primitive_only => return Ty::String,
      IntersectionBuilderState::String(Some(s)) if primitive_only => return Ty::StringLiteral(s),
      IntersectionBuilderState::Symbol(None) if primitive_only => return Ty::Symbol,
      IntersectionBuilderState::Symbol(Some(s)) if primitive_only => return Ty::UniqueSymbol(s),
      IntersectionBuilderState::ObjectKeyword if primitive_only => return Ty::Object,
      IntersectionBuilderState::Void if primitive_only => return Ty::Void,

      // With complex types
      IntersectionBuilderState::BigInt(b) => IntersectionBaseKind::BigInt(b),
      IntersectionBuilderState::Boolean(b) => IntersectionBaseKind::Boolean(b),
      IntersectionBuilderState::Number(n) => IntersectionBaseKind::Number(n),
      IntersectionBuilderState::String(s) => IntersectionBaseKind::String(s),
      IntersectionBuilderState::Symbol(s) => IntersectionBaseKind::Symbol(s),
      IntersectionBuilderState::ObjectKeyword => IntersectionBaseKind::ObjectKeyword,
      IntersectionBuilderState::Void => IntersectionBaseKind::Void,
      IntersectionBuilderState::ObjectLike => {
        if object_like.len() == 1 {
          return object_like[0];
        } else {
          IntersectionBaseKind::NoBase
        }
      }
    };
    Ty::Intersection(allocator.alloc(IntersectionType { kind, object_like }))
  }

  pub fn build(self, allocator: &'a Allocator) -> Ty<'a> {
    let Self { kind, object_like, unresolved, union } = self;
    let base = Self::build_without_union_and_unresolved(allocator, kind, object_like);
    if base == Ty::Never {
      return Ty::Never;
    }
    let with_unresolved = if unresolved.is_empty() {
      base
    } else {
      Ty::Unresolved(UnresolvedType::Intersection(
        allocator.alloc(UnresolvedIntersection { base, unresolved }),
      ))
    };
    if let Some(union) = union {
      into_union(
        allocator,
        union.into_iter().map(|mut builder| {
          builder.add(with_unresolved);
          builder.build(allocator)
        }),
      )
    } else {
      with_unresolved
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntersectionBaseKind<'a> {
  NoBase,
  Number(Option<F64WithEq>),
  String(Option<&'a Atom<'a>>),
  Boolean(Option<bool>),
  Symbol(Option<SymbolId>),
  BigInt(Option<&'a Atom<'a>>),
  ObjectKeyword,
  Void,
}

#[derive(Debug, Clone)]
pub struct IntersectionType<'a> {
  pub kind: IntersectionBaseKind<'a>,
  /// non empty
  pub object_like: Vec<Ty<'a>>,
}

impl<'a> IntersectionType<'a> {
  pub fn kind_to_ty(&self) -> Option<Ty<'a>> {
    match self.kind {
      IntersectionBaseKind::NoBase => None,
      IntersectionBaseKind::Number(Some(n)) => Some(Ty::NumericLiteral(n)),
      IntersectionBaseKind::Number(None) => Some(Ty::Number),
      IntersectionBaseKind::String(Some(s)) => Some(Ty::StringLiteral(s)),
      IntersectionBaseKind::String(None) => Some(Ty::String),
      IntersectionBaseKind::Boolean(Some(b)) => Some(Ty::BooleanLiteral(b)),
      IntersectionBaseKind::Boolean(None) => Some(Ty::Boolean),
      IntersectionBaseKind::Symbol(Some(s)) => Some(Ty::UniqueSymbol(s)),
      IntersectionBaseKind::Symbol(None) => Some(Ty::Symbol),
      IntersectionBaseKind::BigInt(Some(b)) => Some(Ty::BigIntLiteral(b)),
      IntersectionBaseKind::BigInt(None) => Some(Ty::BigInt),
      IntersectionBaseKind::ObjectKeyword => Some(Ty::Object),
      IntersectionBaseKind::Void => Some(Ty::Void),
    }
  }

  pub fn for_each(&self, mut f: impl FnMut(Ty<'a>) -> ()) {
    self.kind_to_ty().map(&mut f);
    for ty in &self.object_like {
      f(*ty);
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn print_intersection_type(&self, intersection: &IntersectionType<'a>) -> TSType<'a> {
    let mut types = self.ast_builder.vec();
    intersection.for_each(|ty| types.push(self.print_type(ty)));
    self.ast_builder.ts_type_intersection_type(SPAN, types)
  }
}

pub fn into_intersection<'a, Iter>(
  allocator: &'a Allocator,
  types: impl IntoIterator<Item = Ty<'a>, IntoIter = Iter>,
) -> Ty<'a>
where
  Iter: Iterator<Item = Ty<'a>> + ExactSizeIterator,
{
  let mut iter = types.into_iter();
  match iter.len() {
    0 => unreachable!(),
    1 => iter.next().unwrap(),
    _ => {
      let mut builder = IntersectionTypeBuilder::default();
      iter.for_each(|ty| builder.add(ty));
      builder.build(allocator)
    }
  }
}
