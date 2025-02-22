use ::core::marker::Sized;

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

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
