use ::core::cmp::Ordering;

use crate::impls::unstable::SpecCore;
use crate::impls::unstable::SpecSint;
use crate::impls::unstable::SpecUint;
use crate::macros::maybe_intrinsic;
use crate::macros::specialize;

#[cfg(not(feature = "core_intrinsics"))]
macro_rules! three_way_compare {
  ($lhs:ident, $rhs:ident) => {
    if $lhs < $rhs {
      Ordering::Less
    } else if $lhs != $rhs {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  };
}

specialize! {
  impl const SpecCore for Int<u8|u16|u32|u64|u128|i8|i16|i32|i64|i128> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn band(lhs: Self, rhs: Self) -> Self {
      lhs & rhs
    }

    #[inline]
    fn bor(lhs: Self, rhs: Self) -> Self {
      lhs | rhs
    }

    #[inline]
    fn bxor(lhs: Self, rhs: Self) -> Self {
      lhs ^ rhs
    }

    #[inline]
    fn bnot(integer: Self) -> Self {
      !integer
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn eq(lhs: Self, rhs: Self) -> bool {
      lhs == rhs
    }

    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------

    #[inline]
    fn swap1(integer: Self) -> Self {
      integer.reverse_bits()
    }

    #[inline]
    fn swap8(integer: Self) -> Self {
      integer.swap_bytes()
    }

    #[inline]
    fn rotl(integer: Self, bits: u32) -> Self {
      integer.rotate_left(bits)
    }

    #[inline]
    fn rotr(integer: Self, bits: u32) -> Self {
      integer.rotate_right(bits)
    }

    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn ctpop(integer: Self) -> u32 {
      integer.count_ones()
    }

    #[inline]
    fn ctlz(integer: Self) -> u32 {
      integer.leading_zeros()
    }

    #[inline]
    fn cttz(integer: Self) -> u32 {
      integer.trailing_zeros()
    }

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

    #[inline]
    fn wrapping_add(lhs: Self, rhs: Self) -> Self {
      lhs.wrapping_add(rhs)
    }

    #[inline]
    fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
      lhs.wrapping_sub(rhs)
    }

    #[inline]
    fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
      lhs.wrapping_mul(rhs)
    }
  }
}

specialize! {
  impl const SpecSint for Int<i8|i16|i32|i64|i128> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

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

    #[inline]
    fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_add(rhs)
    }

    #[inline]
    fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_sub(rhs)
    }

    #[inline]
    fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_mul(rhs)
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_add(rhs)
    }

    #[inline]
    fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_sub(rhs)
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    #[inline]
    unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_add(rhs) }
    }

    #[inline]
    unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_sub(rhs) }
    }

    #[inline]
    unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_mul(rhs) }
    }

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
  impl const SpecUint for Int<u8|u16|u32|u64|u128> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    #[cfg(feature = "disjoint_bitor")]
    #[inline]
    unsafe fn disjoint_bor(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      maybe_intrinsic! {
        @enabled => unsafe {
          ::core::intrinsics::disjoint_bitor(lhs, rhs)
        }
        @default => unsafe {
          lhs.unchecked_disjoint_bitor(rhs)
        }
      }
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

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

    #[inline]
    fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_add(rhs)
    }

    #[inline]
    fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_sub(rhs)
    }

    #[inline]
    fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
      lhs.overflowing_mul(rhs)
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_add(rhs)
    }

    #[inline]
    fn saturating_usub(lhs: Self, rhs: Self) -> Self {
      lhs.saturating_sub(rhs)
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    #[inline]
    unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_add(rhs) }
    }

    #[inline]
    unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_sub(rhs) }
    }

    #[inline]
    unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { lhs.unchecked_mul(rhs) }
    }

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
