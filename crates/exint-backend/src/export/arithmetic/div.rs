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

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
