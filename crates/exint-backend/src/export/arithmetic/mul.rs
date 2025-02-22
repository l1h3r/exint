use ::core::marker::Sized;

use crate::macros::specialize;
use crate::traits::Cast;
use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecUmul: Sized {
  fn omul(self, other: Self) -> (Self, bool);
  fn wmul(self, other: Self) -> Self;
  unsafe fn umul(self, other: Self) -> Self;
}

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecSmul: SpecUmul {
  fn omul(self, other: Self) -> (Self, bool);
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

specialize! {
  impl SpecUmul for Int<1|2|4|8|16> {
    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn omul(self, other: Self) -> (Self, bool) {
      ::core::intrinsics::mul_with_overflow(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wmul(self, other: Self) -> Self {
      ::core::intrinsics::wrapping_mul(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `mul nuw $type` instruction
    #[inline]
    unsafe fn umul(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_mul(self.ucast(), other.ucast()).ucast()
      }
    }
  }
}

specialize! {
  impl SpecSmul for Int<1|2|4|8|16> {
    // LLVM generates `@llvm.smul.with.overflow.$type` intrinsic
    #[inline]
    fn omul(self, other: Self) -> (Self, bool) {
      ::core::intrinsics::mul_with_overflow(self.scast(), other.scast()).scast()
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wmul(self, other: Self) -> Self {
      ::core::intrinsics::wrapping_mul(self.scast(), other.scast()).scast()
    }

    // LLVM generates `mul nsw $type` instruction
    #[inline]
    unsafe fn umul(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_mul(self.scast(), other.scast()).scast()
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
