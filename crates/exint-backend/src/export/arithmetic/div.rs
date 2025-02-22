use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecUdiv {
  unsafe fn udiv(self, rhs: Self) -> Self;
  unsafe fn urem(self, rhs: Self) -> Self;
  unsafe fn ediv(self, rhs: Self) -> Self;
}

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecSdiv: SpecUdiv {
  unsafe fn udiv(self, rhs: Self) -> Self;
  unsafe fn urem(self, rhs: Self) -> Self;
  unsafe fn ediv(self, rhs: Self) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecUdiv for Int<S> {
  default unsafe fn udiv(self, other: Self) -> Self {
    ::core::panic!("SpecUdiv::udiv")
  }

  default unsafe fn urem(self, other: Self) -> Self {
    ::core::panic!("SpecUdiv::urem")
  }

  default unsafe fn ediv(self, other: Self) -> Self {
    ::core::panic!("SpecUdiv::ediv")
  }
}

impl<const S: usize> const SpecSdiv for Int<S> {
  default unsafe fn udiv(self, other: Self) -> Self {
    ::core::panic!("SpecSdiv::udiv")
  }

  default unsafe fn urem(self, other: Self) -> Self {
    ::core::panic!("SpecSdiv::urem")
  }

  default unsafe fn ediv(self, other: Self) -> Self {
    ::core::panic!("SpecSdiv::ediv")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
