use crate::macros::specialize;
use crate::traits::Cast;
use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecConvert {
  fn swap1(self) -> Self;
  fn swap8(self) -> Self;
  fn rotl(self, bits: u32) -> Self;
  fn rotr(self, bits: u32) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecConvert for Int<S> {
  default fn swap1(self) -> Self {
    ::core::panic!("SpecConvert::swap1")
  }

  default fn swap8(self) -> Self {
    ::core::panic!("SpecConvert::swap8")
  }

  default fn rotl(self, bits: u32) -> Self {
    ::core::panic!("SpecConvert::rotl")
  }

  default fn rotr(self, bits: u32) -> Self {
    ::core::panic!("SpecConvert::rotr")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecConvert for Int<1|2|4|8|16> {
    #[inline]
    fn swap1(self) -> Self {
      ::core::intrinsics::bitreverse(self.ucast()).ucast()
    }

    #[inline]
    fn swap8(self) -> Self {
      ::core::intrinsics::bswap(self.ucast()).ucast()
    }

    #[inline]
    fn rotl(self, bits: u32) -> Self {
      ::core::intrinsics::rotate_left(self.ucast(), bits).ucast()
    }

    #[inline]
    fn rotr(self, bits: u32) -> Self {
      ::core::intrinsics::rotate_right(self.ucast(), bits).ucast()
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
