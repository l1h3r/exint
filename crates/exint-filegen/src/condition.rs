use crate::type_name::TypeName;

pub(crate) enum Condition {
  All,
  Gt(usize),
  Ge(usize),
  Lt(usize),
  Le(usize),
  Uint(bool),
  Func(for<'a> fn(&'a TypeName) -> bool),
}

impl Condition {
  pub(crate) fn accept(&self, kind: &TypeName) -> bool {
    match self {
      Self::All => true,
      Self::Gt(bits) => kind.bits() > *bits,
      Self::Ge(bits) => kind.bits() >= *bits,
      Self::Lt(bits) => kind.bits() < *bits,
      Self::Le(bits) => kind.bits() <= *bits,
      Self::Uint(uint) => kind.uint() == *uint,
      Self::Func(func) => func(kind),
    }
  }
}
