use ::core::cmp::Ordering;

use crate::impls::unstable::SpecCore;
use crate::impls::unstable::SpecSint;
use crate::impls::unstable::SpecUint;
use crate::macros::specialize;
use crate::traits::Consts as _;
use crate::traits::Exts;
use crate::traits::Trunc as _;

specialize! {
  impl const SpecCore for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn band(lhs: Self, rhs: Self) -> Self {
      SpecCore::band(lhs.zext(), rhs.zext()).trunc()
    }

    #[inline]
    fn bor(lhs: Self, rhs: Self) -> Self {
      SpecCore::bor(lhs.zext(), rhs.zext()).trunc()
    }

    #[inline]
    fn bxor(lhs: Self, rhs: Self) -> Self {
      SpecCore::bxor(lhs.zext(), rhs.zext()).trunc()
    }

    #[inline]
    fn bnot(integer: Self) -> Self {
      (SpecCore::bnot(integer.zext()) & Self::UMAX.zext()).trunc()
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn eq(lhs: Self, rhs: Self) -> bool {
      SpecCore::eq(lhs.zext(), rhs.zext())
    }

    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------

    // Note: LLVM only recognizes this pattern when increasing the loop unroll
    //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
    //
    // Note: Bad optimization when `N >= 72`, where N is the width of `Self` in bits.
    #[inline]
    fn swap1(integer: Self) -> Self {
      let mut input: <Self as Exts>::Uint = integer.zext();
      let mut value: <Self as Exts>::Uint = 0;
      let mut index: usize = 0;

      while index < Self::BITS as usize {
        value = (value << 1) | (input & 1);
        input >>= 1;
        index += 1;
      }

      value.trunc()
    }

    #[inline]
    fn swap8(integer: Self) -> Self {
      (SpecCore::swap8(integer.zext() << Self::UDIFF)).trunc()
    }

    // TODO: Figure out how to get `@llvm.fshl.$type`
    #[inline]
    fn rotl(integer: Self, bits: u32) -> Self {
      let lhs_shift: u32 = bits % Self::BITS;
      let rhs_shift: u32 = (Self::BITS - lhs_shift) % Self::BITS;

      // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
      let lhs_value: Self = unsafe { SpecCore::unchecked_shl(integer, lhs_shift) };
      // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
      let rhs_value: Self = unsafe { SpecUint::unchecked_lshr(integer, rhs_shift) };

      SpecCore::bor(lhs_value, rhs_value)
    }

    // TODO: Figure out how to get `@llvm.fshr.$type`
    #[inline]
    fn rotr(integer: Self, bits: u32) -> Self {
      let lhs_shift: u32 = bits % Self::BITS;
      let rhs_shift: u32 = (Self::BITS - lhs_shift) % Self::BITS;

      // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
      let lhs_value: Self = unsafe { SpecUint::unchecked_lshr(integer, lhs_shift) };
      // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
      let rhs_value: Self = unsafe { SpecCore::unchecked_shl(integer, rhs_shift) };

      SpecCore::bor(lhs_value, rhs_value)
    }

    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn ctpop(integer: Self) -> u32 {
      SpecCore::ctpop(integer.zext())
    }

    // Note: LLVM only recognizes this pattern when increasing the loop unroll
    //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
    #[inline]
    fn ctlz(integer: Self) -> u32 {
      SpecCore::cttz(SpecCore::swap1(integer))
    }

    #[inline]
    fn cttz(integer: Self) -> u32 {
      if integer.zext() == 0 {
        return Self::BITS;
      }

      SpecCore::cttz(integer.zext())
    }

    #[inline]
    unsafe fn ctlz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCore::cttz_nonzero(SpecCore::swap1(integer)) }
    }

    #[inline]
    unsafe fn cttz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCore::cttz_nonzero(integer.zext()) }
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // TODO: Figure out how to get `shl $type`
    #[inline]
    unsafe fn unchecked_shl(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCore::unchecked_shl(integer.zext(), bits) }.trunc()
    }

    // -------------------------------------------------------------------------
    // Wrapping Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn wrapping_add(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let out: <Self as Exts>::Uint = SpecCore::wrapping_add(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }

    #[inline]
    fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let out: <Self as Exts>::Uint = SpecCore::wrapping_sub(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }

    #[inline]
    fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let out: <Self as Exts>::Uint = SpecCore::wrapping_mul(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }
  }
}

specialize! {
  impl const SpecSint for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn scmp(lhs: Self, rhs: Self) -> Ordering {
      SpecSint::scmp(lhs.sext(), rhs.sext())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
      let lhs: <Self as Exts>::Sint = lhs.sext();
      let rhs: <Self as Exts>::Sint = rhs.sext();

      // SAFETY: Signed addition cannot overflow the larger built-in type.
      let out: <Self as Exts>::Sint = unsafe {
        SpecSint::unchecked_sadd(lhs, rhs)
      };

      (out.trunc(), out > Self::SMAX.sext() || out < Self::SMIN.sext())
    }

    #[inline]
    fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
      let lhs: <Self as Exts>::Sint = lhs.sext();
      let rhs: <Self as Exts>::Sint = rhs.sext();

      // SAFETY: Signed subtraction cannot overflow the larger built-in type.
      let out: <Self as Exts>::Sint = unsafe {
        SpecSint::unchecked_ssub(lhs, rhs)
      };

      (out.trunc(), out > Self::SMAX.sext() || out < Self::SMIN.sext())
    }

    #[inline]
    fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
      let lhs: <Self as Exts>::Sint = lhs.sext();
      let rhs: <Self as Exts>::Sint = rhs.sext();
      let (out, cmp): (<Self as Exts>::Sint, bool) = lhs.overflowing_mul(rhs);

      (out.trunc(), cmp || out > Self::SMAX.sext() || out < Self::SMIN.sext())
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Sint = lhs.sext();
      let rhs: <Self as Exts>::Sint = rhs.sext();

      // SAFETY: Signed addition cannot overflow the larger built-in type.
      let out: <Self as Exts>::Sint = unsafe {
        SpecSint::unchecked_sadd(lhs, rhs)
      };

      if out > Self::SMAX.sext() {
        Self::SMAX
      } else if out < Self::SMIN.sext() {
        Self::SMIN
      } else {
        out.trunc()
      }
    }

    #[inline]
    fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Sint = lhs.sext();
      let rhs: <Self as Exts>::Sint = rhs.sext();

      // SAFETY: Signed subtraction cannot overflow the larger built-in type.
      let out: <Self as Exts>::Sint = unsafe {
        SpecSint::unchecked_ssub(lhs, rhs)
      };

      if out > Self::SMAX.sext() {
        Self::SMAX
      } else if out < Self::SMIN.sext() {
        Self::SMIN
      } else {
        out.trunc()
      }
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // TODO: Figure out how to get `nsw` keyword
    #[inline]
    unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_add(lhs, rhs)
    }

    // TODO: Figure out how to get `nsw` keyword
    #[inline]
    unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_sub(lhs, rhs)
    }

    // TODO: Figure out how to get `nsw` keyword
    #[inline]
    unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_mul(lhs, rhs)
    }

    // TODO: Figure out how to get `sdiv $type`
    //
    // Note: LLVM does not actually seem to support narrowing sdiv
    #[inline]
    unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_sdiv(lhs.sext(), rhs.sext()) }.trunc()
    }

    // TODO: Figure out how to get `srem $type`
    //
    // Note: LLVM does not actually seem to support narrowing srem
    #[inline]
    unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_srem(lhs.sext(), rhs.sext()) }.trunc()
    }

    // TODO: Figure out how to get `ashr $type`
    #[inline]
    unsafe fn unchecked_ashr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_ashr(integer.sext(), bits) }.trunc()
    }
  }
}

specialize! {
  impl const SpecUint for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    // TODO: Figure out how to get `disjoint` keyword
    #[cfg(feature = "disjoint_bitor")]
    #[inline]
    unsafe fn disjoint_bor(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::disjoint_bor(lhs.zext(), rhs.zext()) }.trunc()
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn ucmp(lhs: Self, rhs: Self) -> Ordering {
      SpecUint::ucmp(lhs.zext(), rhs.zext())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add $type` and `icmp ult $type` instructions
    //
    // Note: This appears to be the recommended pattern:
    //   https://github.com/rust-lang/rust/pull/124114#issuecomment-2066173305
    #[inline]
    fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
      let out: Self = SpecCore::wrapping_add(lhs, rhs);
      let cmp: bool = SpecUint::ucmp(out, lhs).is_lt();

      (out, cmp)
    }

    // Note: LLVM generates `sub $type` and `icmp ult $type` which is
    //       considered the canonical form of `usub.with.overflow`.
    //       More info here: https://github.com/rust-lang/rust/pull/103299
    #[inline]
    fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
      // Note: This order (cmp -> sub) is important for i24,i40,i48,i56
      if ::core::matches!(Self::SIZE, 3 | 5 | 6 | 7) {
        let cmp: bool = SpecUint::ucmp(lhs, rhs).is_lt();
        let out: Self = SpecCore::wrapping_sub(lhs, rhs);

        (out, cmp)
      } else {
        let out: Self = SpecCore::wrapping_sub(lhs, rhs);
        let cmp: bool = SpecUint::ucmp(lhs, rhs).is_lt();

        (out, cmp)
      }
    }

    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let (out, cmp): (<Self as Exts>::Uint, bool) = SpecUint::overflowing_umul(lhs, rhs);

      (out.trunc(), cmp || out > Self::UMAX.zext())
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
      let out: Self = SpecCore::wrapping_add(lhs, rhs);
      let cmp: bool = SpecUint::ucmp(out, lhs).is_lt();

      if cmp {
        return Self::UMAX;
      }

      out
    }

    #[inline]
    fn saturating_usub(lhs: Self, rhs: Self) -> Self {
      if SpecUint::ucmp(lhs, rhs).is_lt() {
        return Self::UMIN;
      }

      SpecCore::wrapping_sub(lhs, rhs)
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // TODO: Figure out how to get `nuw` keyword
    #[inline]
    unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_add(lhs, rhs)
    }

    // TODO: Figure out how to get `nuw` keyword
    #[inline]
    unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_sub(lhs, rhs)
    }

    // TODO: Figure out how to get `nuw` keyword
    #[inline]
    unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_mul(lhs, rhs)
    }

    #[inline]
    unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_udiv(lhs.zext(), rhs.zext()) }.trunc()
    }

    #[inline]
    unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_urem(lhs.zext(), rhs.zext()) }.trunc()
    }

    // TODO: Figure out how to get `lshr $type`
    #[inline]
    unsafe fn unchecked_lshr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_lshr(integer.zext(), bits) }.trunc()
    }
  }
}
