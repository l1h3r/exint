use ::core::cmp::Ordering;

use crate::llapi::impls::unstable::SpecCoreFuncs;
use crate::llapi::impls::unstable::SpecSintFuncs;
use crate::llapi::impls::unstable::SpecUintFuncs;
use crate::llapi::macros::maybe_intrinsic;
use crate::llapi::macros::specialize;

#[cfg(not(feature = "core_intrinsics"))]
macro_rules! three_way_compare {
  ($lhs:ident, $rhs:ident) => {
    if $lhs < $rhs {
      ::core::cmp::Ordering::Less
    } else if $lhs != $rhs {
      ::core::cmp::Ordering::Greater
    } else {
      ::core::cmp::Ordering::Equal
    }
  };
}

specialize! {
  impl const SpecCoreFuncs for Int<u8|u16|u32|u64|u128|i8|i16|i32|i64|i128> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    // LLVM generates `and $type` instruction
    #[inline]
    fn band(lhs: Self, rhs: Self) -> Self {
      lhs & rhs
    }

    // LLVM generates `or $type` instruction
    #[inline]
    fn bor(lhs: Self, rhs: Self) -> Self {
      lhs | rhs
    }

    // LLVM generates `xor $type` instruction
    #[inline]
    fn bxor(lhs: Self, rhs: Self) -> Self {
      lhs ^ rhs
    }

    // LLVM generates `xor $type .. -1` instruction
    #[inline]
    fn bnot(integer: Self) -> Self {
      !integer
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp eq $type` instruction
    #[inline]
    fn eq(lhs: Self, rhs: Self) -> bool {
      lhs == rhs
    }

    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    #[inline]
    fn swap1(integer: Self) -> Self {
      integer.reverse_bits()
    }

    // LLVM generates `@llvm.bswap.$type` intrinsic
    #[inline]
    fn swap8(integer: Self) -> Self {
      integer.swap_bytes()
    }

    // LLVM generates `@llvm.fshl.$type` intrinsic
    #[inline]
    fn rotl(integer: Self, bits: u32) -> Self {
      integer.rotate_left(bits)
    }

    // LLVM generates `@llvm.fshr.$type` intrinsic
    #[inline]
    fn rotr(integer: Self, bits: u32) -> Self {
      integer.rotate_right(bits)
    }

    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(integer: Self) -> u32 {
      integer.count_ones()
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    #[inline]
    fn ctlz(integer: Self) -> u32 {
      integer.leading_zeros()
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(integer: Self) -> u32 {
      integer.trailing_zeros()
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn ctlz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::ctlz_nonzero(integer)
        }
        @default => unsafe {
          ::core::num::NonZero::new_unchecked(integer).leading_zeros()
        }
      }
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn cttz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::cttz_nonzero(integer)
        }
        @default => unsafe {
          ::core::num::NonZero::new_unchecked(integer).trailing_zeros()
        }
      }
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `shl $type` instruction
    #[inline]
    unsafe fn unchecked_shl(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_shl(integer, bits)
        }
        @default => unsafe {
          // TODO: Use `unchecked_shl` when stable.
          integer.checked_shl(bits).unwrap_unchecked()
        }
      }
    }

    // -------------------------------------------------------------------------
    // Wrapping Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add $type` instruction
    #[inline]
    fn wrapping_add(lhs: Self, rhs: Self) -> Self {
      lhs.wrapping_add(rhs)
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
      lhs.wrapping_sub(rhs)
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
      lhs.wrapping_mul(rhs)
    }
  }
}

specialize! {
  impl const SpecSintFuncs for Int<i8|i16|i32|i64|i128> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp slt $type` and `icmp ne $type` instructions
    #[inline]
    fn scmp(lhs: Self, rhs: Self) -> Ordering {
      maybe_intrinsic! {
        @enabled => {
          ::core::intrinsics::three_way_compare(lhs, rhs)
        }
        @default => {
          three_way_compare!(lhs, rhs)
        }
      }
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.sadd.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_add(rhs)
    }

    // LLVM generates `@llvm.ssub.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_sub(rhs)
    }

    // LLVM generates `@llvm.smul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_mul(rhs)
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.sadd.sat.$type` intrinsic
    #[inline]
    fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_add(rhs)
    }

    // LLVM generates `@llvm.ssub.sat.$type` intrinsic
    #[inline]
    fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_sub(rhs)
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add nsw $type` instruction
    #[inline]
    unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_add(rhs) }
    }

    // LLVM generates `sub nsw $type` instruction
    #[inline]
    unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_sub(rhs) }
    }

    // LLVM generates `mul nsw $type` instruction
    #[inline]
    unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_mul(rhs) }
    }

    // LLVM generates `sdiv $type` instruction
    #[inline]
    unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_div(lhs, rhs)
        }
        @default => unsafe {
          // TODO: Use `unchecked_div` when implemented and stable.
          lhs.checked_div(rhs).unwrap_unchecked()
        }
      }
    }

    // LLVM generates `srem $type` instruction
    #[inline]
    unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_rem(lhs, rhs)
        }
        @default => unsafe {
          // TODO: Use `unchecked_rem` when implemented and stable.
          lhs.checked_rem(rhs).unwrap_unchecked()
        }
      }
    }

    // LLVM generates `ashr $type` instruction
    #[inline]
    unsafe fn unchecked_ashr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_shr(integer, bits)
        }
        @default => unsafe {
          // TODO: Use `unchecked_shr` when stable.
          integer.checked_shr(bits).unwrap_unchecked()
        }
      }
    }
  }
}

specialize! {
  impl const SpecUintFuncs for Int<u8|u16|u32|u64|u128> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp ult $type` and `icmp ne $type` instructions
    #[inline]
    fn ucmp(lhs: Self, rhs: Self) -> Ordering {
      maybe_intrinsic! {
        @enabled => {
          ::core::intrinsics::three_way_compare(lhs, rhs)
        }
        @default => {
          three_way_compare!(lhs, rhs)
        }
      }
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.uadd.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_add(rhs)
    }

    // LLVM generates `sub $type` instruction
    //
    // Note: `@llvm.usub.with.overflow.$type` is not emitted by rustc:
    //   https://github.com/rust-lang/rust/pull/103299
    #[inline]
    fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_sub(rhs)
    }

    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_mul(rhs)
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.uadd.sat.$type` intrinsic
    #[inline]
    fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_add(rhs)
    }

    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn saturating_usub(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_sub(rhs)
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add nuw $type` instruction
    #[inline]
    unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_add(rhs) }
    }

    // LLVM generates `sub nuw $type` instruction
    #[inline]
    unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_sub(rhs) }
    }

    // LLVM generates `mul nuw $type` instruction
    #[inline]
    unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_mul(rhs) }
    }

    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_div(lhs, rhs)
        }
        @default => unsafe {
          // TODO: Use `unchecked_div` when implemented and stable.
          lhs.checked_div(rhs).unwrap_unchecked()
        }
      }
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_rem(lhs, rhs)
        }
        @default => unsafe {
          // TODO: Use `unchecked_rem` when implemented and stable.
          lhs.checked_rem(rhs).unwrap_unchecked()
        }
      }
    }

    // LLVM generates `lshr $type` instruction
    #[inline]
    unsafe fn unchecked_lshr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::unchecked_shr(integer, bits)
        }
        @default => unsafe {
          // TODO: Use `unchecked_shr` when stable.
          integer.checked_shr(bits).unwrap_unchecked()
        }
      }
    }
  }
}
