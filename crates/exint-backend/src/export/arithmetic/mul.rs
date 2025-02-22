use ::core::marker::Sized;

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

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
