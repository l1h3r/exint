use crate::macros::specialize;
use crate::traits::Cast;
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

specialize! {
  impl SpecShift for Int<1|2|4|8|16> {
    // LLVM generates `shl $type` instruction
    #[inline]
    unsafe fn shl(self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_shl(self.ucast(), bits).ucast()
      }
    }

    // LLVM generates `lshr $type` instruction
    #[inline]
    unsafe fn lshr(self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_shr(self.ucast(), bits).ucast()
      }
    }

    // LLVM generates `ashr $type` instruction
    #[inline]
    unsafe fn ashr(self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_shr(self.scast(), bits).scast()
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
