use ::core::cmp::Ordering;

use crate::impls::unstable::SpecCore;
use crate::impls::unstable::SpecSint;
use crate::impls::unstable::SpecUint;
use crate::macros::specialize;
use crate::traits::Cast as _;

specialize! {
  impl const SpecCore for Int<1|2|4|8|16> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn band(lhs: Self, rhs: Self) -> Self {
      SpecCore::band(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn bor(lhs: Self, rhs: Self) -> Self {
      SpecCore::bor(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn bxor(lhs: Self, rhs: Self) -> Self {
      SpecCore::bxor(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn bnot(integer: Self) -> Self {
      SpecCore::bnot(integer.ucast()).ucast()
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn eq(lhs: Self, rhs: Self) -> bool {
      SpecCore::eq(lhs.ucast(), rhs.ucast())
    }

    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------

    #[inline]
    fn swap1(integer: Self) -> Self {
      SpecCore::swap1(integer.ucast()).ucast()
    }

    #[inline]
    fn swap8(integer: Self) -> Self {
      SpecCore::swap8(integer.ucast()).ucast()
    }

    #[inline]
    fn rotl(integer: Self, bits: u32) -> Self {
      SpecCore::rotl(integer.ucast(), bits).ucast()
    }

    #[inline]
    fn rotr(integer: Self, bits: u32) -> Self {
      SpecCore::rotr(integer.ucast(), bits).ucast()
    }

    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn ctpop(integer: Self) -> u32 {
      SpecCore::ctpop(integer.ucast())
    }

    #[inline]
    fn ctlz(integer: Self) -> u32 {
      SpecCore::ctlz(integer.ucast())
    }

    #[inline]
    fn cttz(integer: Self) -> u32 {
      SpecCore::cttz(integer.ucast())
    }

    #[inline]
    unsafe fn ctlz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCore::ctlz_nonzero(integer.ucast()) }
    }

    #[inline]
    unsafe fn cttz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCore::cttz_nonzero(integer.ucast()) }
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    #[inline]
    unsafe fn unchecked_shl(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCore::unchecked_shl(integer.ucast(), bits) }.ucast()
    }

    // -------------------------------------------------------------------------
    // Wrapping Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn wrapping_add(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_add(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_sub(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
      SpecCore::wrapping_mul(lhs.ucast(), rhs.ucast()).ucast()
    }
  }
}

specialize! {
  impl const SpecSint for Int<1|2|4|8|16> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn scmp(lhs: Self, rhs: Self) -> Ordering {
      SpecSint::scmp(lhs.scast(), rhs.scast())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecSint::overflowing_sadd(lhs.scast(), rhs.scast()).scast()
    }

    #[inline]
    fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecSint::overflowing_ssub(lhs.scast(), rhs.scast()).scast()
    }

    #[inline]
    fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecSint::overflowing_smul(lhs.scast(), rhs.scast()).scast()
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
      SpecSint::saturating_sadd(lhs.scast(), rhs.scast()).scast()
    }

    #[inline]
    fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
      SpecSint::saturating_ssub(lhs.scast(), rhs.scast()).scast()
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    #[inline]
    unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_sadd(lhs.scast(), rhs.scast()) }.scast()
    }

    #[inline]
    unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_ssub(lhs.scast(), rhs.scast()) }.scast()
    }

    #[inline]
    unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_smul(lhs.scast(), rhs.scast()) }.scast()
    }

    #[inline]
    unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_sdiv(lhs.scast(), rhs.scast()) }.scast()
    }

    #[inline]
    unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_srem(lhs.scast(), rhs.scast()) }.scast()
    }

    #[inline]
    unsafe fn unchecked_ashr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSint::unchecked_ashr(integer.scast(), bits) }.scast()
    }
  }
}

specialize! {
  impl const SpecUint for Int<1|2|4|8|16> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    #[cfg(feature = "disjoint_bitor")]
    #[inline]
    unsafe fn disjoint_bor(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::disjoint_bor(lhs.ucast(), rhs.ucast()).ucast() }
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn ucmp(lhs: Self, rhs: Self) -> Ordering {
      SpecUint::ucmp(lhs.ucast(), rhs.ucast())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecUint::overflowing_uadd(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecUint::overflowing_usub(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecUint::overflowing_umul(lhs.ucast(), rhs.ucast()).ucast()
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    #[inline]
    fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
      SpecUint::saturating_uadd(lhs.ucast(), rhs.ucast()).ucast()
    }

    #[inline]
    fn saturating_usub(lhs: Self, rhs: Self) -> Self {
      SpecUint::saturating_usub(lhs.ucast(), rhs.ucast()).ucast()
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    #[inline]
    unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_uadd(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    #[inline]
    unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_usub(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    #[inline]
    unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_umul(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    #[inline]
    unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_udiv(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    #[inline]
    unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_urem(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    #[inline]
    unsafe fn unchecked_lshr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUint::unchecked_lshr(integer.ucast(), bits) }.ucast()
    }
  }
}
