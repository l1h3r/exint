use ::core::marker::Sized;

use crate::macros::specialize;
use crate::traits::Cast;
use crate::types::Int;

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

impl<const S: usize> const SpecUadd for Int<S> {
  default fn oadd(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecUadd::oadd")
  }

  default fn sadd(self, other: Self) -> Self {
    ::core::panic!("SpecUadd::sadd")
  }

  default fn wadd(self, other: Self) -> Self {
    ::core::panic!("SpecUadd::wadd")
  }

  default unsafe fn uadd(self, other: Self) -> Self {
    ::core::panic!("SpecUadd::uadd")
  }
}

impl<const S: usize> const SpecSadd for Int<S> {
  default fn oadd(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecSadd::oadd")
  }

  default fn sadd(self, other: Self) -> Self {
    ::core::panic!("SpecSadd::sadd")
  }

  default fn wadd(self, other: Self) -> Self {
    ::core::panic!("SpecSadd::wadd")
  }

  default unsafe fn uadd(self, other: Self) -> Self {
    ::core::panic!("SpecSadd::uadd")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecUadd for Int<1|2|4|8|16> {
    // LLVM generates `@llvm.uadd.with.overflow.$type` intrinsic
    #[inline]
    fn oadd(self, other: Self) -> (Self, bool) {
      ::core::intrinsics::add_with_overflow(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `@llvm.uadd.sat.$type` intrinsic
    #[inline]
    fn sadd(self, other: Self) -> Self {
      ::core::intrinsics::saturating_add(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `add $type` instruction
    #[inline]
    fn wadd(self, other: Self) -> Self {
      ::core::intrinsics::wrapping_add(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `add nuw $type` instruction
    #[inline]
    unsafe fn uadd(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_add(self.ucast(), other.ucast()).ucast()
      }
    }
  }
}

specialize! {
  impl SpecSadd for Int<1|2|4|8|16> {
    // LLVM generates `@llvm.sadd.with.overflow.$type` intrinsic
    #[inline]
    fn oadd(self, other: Self) -> (Self, bool) {
      ::core::intrinsics::add_with_overflow(self.scast(), other.scast()).scast()
    }

    // LLVM generates `@llvm.sadd.sat.$type` intrinsic
    #[inline]
    fn sadd(self, other: Self) -> Self {
      ::core::intrinsics::saturating_add(self.scast(), other.scast()).scast()
    }

    // LLVM generates `add $type` instruction
    #[inline]
    fn wadd(self, other: Self) -> Self {
      ::core::intrinsics::wrapping_add(self.scast(), other.scast()).scast()
    }

    // LLVM generates `add nsw $type` instruction
    #[inline]
    unsafe fn uadd(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_add(self.scast(), other.scast()).scast()
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
