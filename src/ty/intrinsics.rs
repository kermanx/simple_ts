use oxc::ast::ast::TSType;

use crate::analyzer::Analyzer;

use super::Ty;

#[derive(Debug, Clone, Copy)]
pub enum IntrinsicType {
  Uppercase,
  Lowercase,
  Capitalize,
  Uncapitalize,
  NoInfer,
}

impl IntrinsicType {
  pub fn from_name(name: &str) -> Self {
    match name {
      "Uppercase" => Self::Uppercase,
      "Lowercase" => Self::Lowercase,
      "Capitalize" => Self::Capitalize,
      "Uncapitalize" => Self::Uncapitalize,
      "NoInfer" => Self::NoInfer,
      _ => unreachable!(),
    }
  }
}

impl<'a> Analyzer<'a> {
  pub fn resolve_intrinsic_type(&mut self, intrinsic: IntrinsicType, args: &[Ty<'a>]) -> Ty<'a> {
    let [arg] = args else {
      return Ty::Error;
    };
    match arg {
      Ty::Error | Ty::Any | Ty::Never | Ty::String => *arg,

      Ty::StringLiteral(s) => {
        let result = match intrinsic {
          IntrinsicType::Uppercase => s.to_uppercase(),
          IntrinsicType::Lowercase => s.to_lowercase(),
          IntrinsicType::Capitalize => {
            let mut chars = s.chars();
            let first = chars.next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
            let rest = chars.as_str().to_string();
            format!("{}{}", first, rest)
          }
          IntrinsicType::Uncapitalize => {
            let mut chars = s.chars();
            let first = chars.next().map(|c| c.to_lowercase().to_string()).unwrap_or_default();
            let rest = chars.as_str().to_string();
            format!("{}{}", first, rest)
          }
          IntrinsicType::NoInfer => todo!(),
        };
        Ty::StringLiteral(self.allocator.alloc_atom(&result))
      }

      _ => Ty::Error,
    }
  }

  pub fn serialize_intrinsic_type(&mut self, intrinsic: IntrinsicType) -> TSType<'a> {
    todo!()
  }
}
