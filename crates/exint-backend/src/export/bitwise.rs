use crate::macros::cmap;
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
    cmap!(@for index in 0..S do self[index] & other[index])
  }

  default fn or(self, other: Self) -> Self {
    cmap!(@for index in 0..S do self[index] | other[index])
  }

  default fn xor(self, other: Self) -> Self {
    cmap!(@for index in 0..S do self[index] ^ other[index])
  }

  default fn not(self) -> Self {
    cmap!(@for index in 0..S do !self[index])
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecBitwise for Int<1|2|4|8|16> {
    // LLVM generates `and $type` instruction
    #[inline]
    fn and(self, other: Self) -> Self {
      (self.ucast() & other.ucast()).ucast()
    }

    // LLVM generates `or $type` instruction
    #[inline]
    fn or(self, other: Self) -> Self {
      (self.ucast() | other.ucast()).ucast()
    }

    // LLVM generates `xor $type` instruction
    #[inline]
    fn xor(self, other: Self) -> Self {
      (self.ucast() ^ other.ucast()).ucast()
    }

    // LLVM generates `xor $type .. -1` instruction
    #[inline]
    fn not(self) -> Self {
      (!self.ucast()).ucast()
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecBitwise for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `and $type` instruction
    #[inline]
    fn and(self, other: Self) -> Self {
      (self.zext() & other.zext()).trunc()
    }

    // LLVM generates `or $type` instruction
    #[inline]
    fn or(self, other: Self) -> Self {
      (self.zext() | other.zext()).trunc()
    }

    // LLVM generates `xor $type` instruction
    #[inline]
    fn xor(self, other: Self) -> Self {
      (self.zext() ^ other.zext()).trunc()
    }

    // LLVM generates `xor $type .. -1` instruction
    #[inline]
    fn not(self) -> Self {
      (!self.zext() & Self::UMASK).trunc()
    }
  }
}
