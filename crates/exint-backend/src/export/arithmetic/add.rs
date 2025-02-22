use ::core::marker::Sized;

use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecUadd: Sized {
  fn oadd(self, other: Self) -> (Self, bool);
  fn sadd(self, other: Self) -> Self;
  fn wadd(self, other: Self) -> Self;
  unsafe fn uadd(self, other: Self) -> Self;
}

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecSadd: SpecUadd {
  fn oadd(self, other: Self) -> (Self, bool);
  fn sadd(self, other: Self) -> Self;
  fn wadd(self, other: Self) -> Self;
  unsafe fn uadd(self, other: Self) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecUadd for Int<S> {
  default fn oadd(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecUadd::oadd")
  }

  default fn sadd(self, other: Self) -> Self {
    ::core::panic!("SpecUadd::sadd")
  }

  default fn wadd(self, other: Self) -> Self {
    ::core::panic!("SpecUadd::wadd")
  }

  default unsafe fn uadd(self, other: Self) -> Self {
    ::core::panic!("SpecUadd::uadd")
  }
}

impl<const S: usize> const SpecSadd for Int<S> {
  default fn oadd(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecSadd::oadd")
  }

  default fn sadd(self, other: Self) -> Self {
    ::core::panic!("SpecSadd::sadd")
  }

  default fn wadd(self, other: Self) -> Self {
    ::core::panic!("SpecSadd::wadd")
  }

  default unsafe fn uadd(self, other: Self) -> Self {
    ::core::panic!("SpecSadd::uadd")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
