use ::core::marker::Sized;

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

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
