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
pub(crate) trait SpecUdiv {
  unsafe fn udiv(self, other: Self) -> Self;
  unsafe fn urem(self, other: Self) -> Self;
  unsafe fn ediv(self, other: Self) -> Self;
}

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecSdiv: SpecUdiv {
  unsafe fn udiv(self, other: Self) -> Self;
  unsafe fn urem(self, other: Self) -> Self;
  unsafe fn ediv(self, other: Self) -> Self;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecUdiv for Int<S> {
  default unsafe fn udiv(self, other: Self) -> Self {
    ::core::panic!("SpecUdiv::udiv")
  }

  default unsafe fn urem(self, other: Self) -> Self {
    ::core::panic!("SpecUdiv::urem")
  }

  default unsafe fn ediv(self, other: Self) -> Self {
    ::core::panic!("SpecUdiv::ediv")
  }
}

impl<const S: usize> const SpecSdiv for Int<S> {
  default unsafe fn udiv(self, other: Self) -> Self {
    ::core::panic!("SpecSdiv::udiv")
  }

  default unsafe fn urem(self, other: Self) -> Self {
    ::core::panic!("SpecSdiv::urem")
  }

  default unsafe fn ediv(self, other: Self) -> Self {
    ::core::panic!("SpecSdiv::ediv")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecUdiv for Int<1|2|4|8|16> {
    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn udiv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_div(self.ucast(), other.ucast()).ucast()
      }
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn urem(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_rem(self.ucast(), other.ucast()).ucast()
      }
    }

    // LLVM generates `udiv exact $type` instruction
    #[inline]
    unsafe fn ediv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::exact_div(self.ucast(), other.ucast()).ucast()
      }
    }
  }
}

specialize! {
  impl SpecSdiv for Int<1|2|4|8|16> {
    // LLVM generates `sdiv $type` instruction
    #[inline]
    unsafe fn udiv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_div(self.scast(), other.scast()).scast()
      }
    }

    // LLVM generates `srem $type` instruction
    #[inline]
    unsafe fn urem(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_rem(self.scast(), other.scast()).scast()
      }
    }

    // LLVM generates `sdiv exact $type` instruction
    #[inline]
    unsafe fn ediv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::exact_div(self.scast(), other.scast()).scast()
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecUdiv for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn udiv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_div(self.zext(), other.zext()).trunc()
      }
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn urem(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::unchecked_rem(self.zext(), other.zext()).trunc()
      }
    }
  }
}
