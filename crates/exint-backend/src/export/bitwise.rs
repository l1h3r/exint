use crate::macros::specialize;
use crate::traits::Cast;
use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecBitwise {
  fn and(self, other: Self) -> Self;
  fn or(self, other: Self) -> Self;
  fn xor(self, other: Self) -> Self;
  fn not(self) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecBitwise for Int<S> {
  default fn and(self, other: Self) -> Self {
    ::core::panic!("SpecBitwise::and")
  }

  default fn or(self, other: Self) -> Self {
    ::core::panic!("SpecBitwise::or")
  }

  default fn xor(self, other: Self) -> Self {
    ::core::panic!("SpecBitwise::xor")
  }

  default fn not(self) -> Self {
    ::core::panic!("SpecBitwise::not")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecBitwise for Int<1|2|4|8|16> {
    #[inline]
    fn and(self, other: Self) -> Self {
      (self.ucast() & other.ucast()).ucast()
    }

    #[inline]
    fn or(self, other: Self) -> Self {
      (self.ucast() | other.ucast()).ucast()
    }

    #[inline]
    fn xor(self, other: Self) -> Self {
      (self.ucast() ^ other.ucast()).ucast()
    }

    #[inline]
    fn not(self) -> Self {
      (!self.ucast()).ucast()
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
