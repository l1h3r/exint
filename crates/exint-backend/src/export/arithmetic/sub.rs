use ::core::marker::Sized;

use crate::export::compare::SpecCompare;
use crate::macros::specialize;
use crate::traits::Cast;
use crate::traits::Exts;
use crate::traits::Trunc;
use crate::types::Int;

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

impl<const S: usize> const SpecUsub for Int<S> {
  default fn osub(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecUsub::osub")
  }

  default fn ssub(self, other: Self) -> Self {
    ::core::panic!("SpecUsub::ssub")
  }

  default fn wsub(self, other: Self) -> Self {
    ::core::panic!("SpecUsub::wsub")
  }

  default unsafe fn usub(self, other: Self) -> Self {
    ::core::panic!("SpecUsub::usub")
  }
}

impl<const S: usize> const SpecSsub for Int<S> {
  default fn osub(self, other: Self) -> (Self, bool) {
    ::core::panic!("SpecSsub::osub")
  }

  default fn ssub(self, other: Self) -> Self {
    ::core::panic!("SpecSsub::ssub")
  }

  default fn wsub(self, other: Self) -> Self {
    ::core::panic!("SpecSsub::wsub")
  }

  default unsafe fn usub(self, other: Self) -> Self {
    ::core::panic!("SpecSsub::usub")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecUsub for Int<1|2|4|8|16> {
    // LLVM generates `sub $type` instruction
    //
    // TODO: Why doesn't this generate `@llvm.usub.with.overflow.$type` ?
    //
    // Note: Not emitted by rustc so ignore above
    //   https://github.com/rust-lang/rust/blob/8c39ce5b4fb5b61796e5fd8cec56c7b9abd2122b/compiler/rustc_codegen_llvm/src/builder.rs#L371
    #[inline]
    fn osub(self, other: Self) -> (Self, bool) {
      ::core::intrinsics::sub_with_overflow(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn ssub(self, other: Self) -> Self {
      ::core::intrinsics::saturating_sub(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      ::core::intrinsics::wrapping_sub(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `sub nuw $type` instruction
    #[inline]
    unsafe fn usub(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_sub(self.ucast(), other.ucast()).ucast()
      }
    }
  }
}

specialize! {
  impl SpecSsub for Int<1|2|4|8|16> {
    // LLVM generates `@llvm.ssub.with.overflow.$type` intrinsic
    #[inline]
    fn osub(self, other: Self) -> (Self, bool) {
      ::core::intrinsics::sub_with_overflow(self.scast(), other.scast()).scast()
    }

    // LLVM generates `@llvm.ssub.sat.$type` intrinsic
    #[inline]
    fn ssub(self, other: Self) -> Self {
      ::core::intrinsics::saturating_sub(self.scast(), other.scast()).scast()
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      ::core::intrinsics::wrapping_sub(self.scast(), other.scast()).scast()
    }

    // LLVM generates `sub nsw $type` instruction
    #[inline]
    unsafe fn usub(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_sub(self.scast(), other.scast()).scast()
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecUsub for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn ssub(self, other: Self) -> Self {
      if SpecCompare::ucmp(self, other).is_lt() {
        return Self::UMIN;
      }

      SpecUsub::wsub(self, other)
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      let lhs: <Self as Exts>::Uint = self.zext();
      let rhs: <Self as Exts>::Uint = other.zext();
      let out: <Self as Exts>::Uint = ::core::intrinsics::wrapping_sub(lhs, rhs);

      (out & Self::UMASK).trunc()
    }
  }
}

specialize! {
  impl SpecSsub for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      SpecUsub::wsub(self, other)
    }
  }
}
