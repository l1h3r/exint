use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecShift {
  unsafe fn shl(self, bits: u32) -> Self;
  unsafe fn lshr(self, bits: u32) -> Self;
  unsafe fn ashr(self, bits: u32) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecShift for Int<S> {
  default unsafe fn shl(self, bits: u32) -> Self {
    ::core::panic!("SpecShift::shl")
  }

  default unsafe fn lshr(self, bits: u32) -> Self {
    ::core::panic!("SpecShift::lshr")
  }

  default unsafe fn ashr(self, bits: u32) -> Self {
    ::core::panic!("SpecShift::ashr")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
