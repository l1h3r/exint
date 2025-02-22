use crate::macros::cfor;
use crate::macros::crev;
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
    crev!(@for index in 0..S do self[index].reverse_bits())
  }

  default fn swap8(self) -> Self {
    crev!(@for index in 0..S do self[index])
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
    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    #[inline]
    fn swap1(self) -> Self {
      ::core::intrinsics::bitreverse(self.ucast()).ucast()
    }

    // LLVM generates `@llvm.bswap.$type` intrinsic
    #[inline]
    fn swap8(self) -> Self {
      ::core::intrinsics::bswap(self.ucast()).ucast()
    }

    // LLVM generates `@llvm.fshl.$type` intrinsic
    #[inline]
    fn rotl(self, bits: u32) -> Self {
      ::core::intrinsics::rotate_left(self.ucast(), bits).ucast()
    }

    // LLVM generates `@llvm.fshr.$type` intrinsic
    #[inline]
    fn rotr(self, bits: u32) -> Self {
      ::core::intrinsics::rotate_right(self.ucast(), bits).ucast()
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecConvert for Int<3|5|6|7> {
    // LLVM generates `@llvm.bswap.$type` intrinsic
    //
    // Note: LLVM only generates this intrinsic when the integer type has an
    //       even number of bytes (positive multiple of 16 bits).
    #[inline]
    fn swap8(self) -> Self {
      (self.zext().swap_bytes() >> Self::UDIFF).trunc()
    }
  }
}

specialize! {
  impl SpecConvert for Int<9|10|11|12|13|14|15> {
    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    //
    // Note: LLVM only recognizes this pattern when increasing the loop unroll
    //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
    #[inline]
    fn swap1(self) -> Self {
      let mut input: u128 = self.zext();
      let mut value: u128 = 0;

      cfor! {
        @for index in 0..(Self::BITS as usize) {
          value = (value << 1) | (input & 1);
          input >>= 1;
        }
      }

      value.trunc()
    }

    // LLVM generates `@llvm.bswap.$type` intrinsic
    //
    // Note: LLVM only generates this intrinsic when the integer type has an
    //       even number of bytes (positive multiple of 16 bits).
    #[inline]
    fn swap8(self) -> Self {
      (self.zext().swap_bytes() >> Self::UDIFF).trunc()
    }
  }
}
