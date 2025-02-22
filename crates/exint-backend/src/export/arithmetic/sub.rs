use ::core::marker::Sized;

use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecUsub: Sized {
  fn osub(self, other: Self) -> (Self, bool);
  fn ssub(self, other: Self) -> Self;
  fn wsub(self, other: Self) -> Self;
  unsafe fn usub(self, other: Self) -> Self;
}

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecSsub: SpecUsub {
  fn osub(self, other: Self) -> (Self, bool);
  fn ssub(self, other: Self) -> Self;
  fn wsub(self, other: Self) -> Self;
  unsafe fn usub(self, other: Self) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecUsub for Int<S> {
  default fn osub(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecUsub::osub")
  }

  default fn ssub(self, other: Self) -> Self {
    ::core::panic!("SpecUsub::ssub")
  }

  default fn wsub(self, other: Self) -> Self {
    ::core::panic!("SpecUsub::wsub")
  }

  default unsafe fn usub(self, other: Self) -> Self {
    ::core::panic!("SpecUsub::usub")
  }
}

impl<const S: usize> const SpecSsub for Int<S> {
  default fn osub(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecSsub::osub")
  }

  default fn ssub(self, other: Self) -> Self {
    ::core::panic!("SpecSsub::ssub")
  }

  default fn wsub(self, other: Self) -> Self {
    ::core::panic!("SpecSsub::wsub")
  }

  default unsafe fn usub(self, other: Self) -> Self {
    ::core::panic!("SpecSsub::usub")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
