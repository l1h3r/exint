use ::core::marker::Sized;

use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecUmul: Sized {
  fn omul(self, other: Self) -> (Self, bool);
  fn smul(self, other: Self) -> Self;
  fn wmul(self, other: Self) -> Self;
  unsafe fn umul(self, other: Self) -> Self;
}

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecSmul: SpecUmul {
  fn omul(self, other: Self) -> (Self, bool);
  fn smul(self, other: Self) -> Self;
  fn wmul(self, other: Self) -> Self;
  unsafe fn umul(self, other: Self) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecUmul for Int<S> {
  default fn omul(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecUmul::omul")
  }

  default fn smul(self, other: Self) -> Self {
    ::core::panic!("SpecUmul::smul")
  }

  default fn wmul(self, other: Self) -> Self {
    ::core::panic!("SpecUmul::wmul")
  }

  default unsafe fn umul(self, other: Self) -> Self {
    ::core::panic!("SpecUmul::umul")
  }
}

impl<const S: usize> const SpecSmul for Int<S> {
  default fn omul(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecSmul::omul")
  }

  default fn smul(self, other: Self) -> Self {
    ::core::panic!("SpecSmul::smul")
  }

  default fn wmul(self, other: Self) -> Self {
    ::core::panic!("SpecSmul::wmul")
  }

  default unsafe fn umul(self, other: Self) -> Self {
    ::core::panic!("SpecSmul::umul")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
