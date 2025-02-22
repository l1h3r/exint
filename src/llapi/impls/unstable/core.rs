use ::core::cmp::Ordering;

use crate::llapi::impls::unstable::SpecCoreFuncs;
use crate::llapi::impls::unstable::SpecSintFuncs;
use crate::llapi::impls::unstable::SpecUintFuncs;
use crate::llapi::macros::specialize;
use crate::llapi::traits::Cast as _;

specialize! {
  impl const SpecCoreFuncs for Int<1|2|4|8|16> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    // LLVM generates `and $type` instruction
    #[inline]
    fn band(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::band(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `or $type` instruction
    #[inline]
    fn bor(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::bor(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `xor $type` instruction
    #[inline]
    fn bxor(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::bxor(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `xor $type .. -1` instruction
    #[inline]
    fn bnot(integer: Self) -> Self {
      SpecCoreFuncs::bnot(integer.ucast()).ucast()
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp eq $type` instruction
    #[inline]
    fn eq(lhs: Self, rhs: Self) -> bool {
      SpecCoreFuncs::eq(lhs.ucast(), rhs.ucast())
    }

    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    #[inline]
    fn swap1(integer: Self) -> Self {
      SpecCoreFuncs::swap1(integer.ucast()).ucast()
    }

    // LLVM generates `@llvm.bswap.$type` intrinsic
    #[inline]
    fn swap8(integer: Self) -> Self {
      SpecCoreFuncs::swap8(integer.ucast()).ucast()
    }

    // LLVM generates `@llvm.fshl.$type` intrinsic
    #[inline]
    fn rotl(integer: Self, bits: u32) -> Self {
      SpecCoreFuncs::rotl(integer.ucast(), bits).ucast()
    }

    // LLVM generates `@llvm.fshr.$type` intrinsic
    #[inline]
    fn rotr(integer: Self, bits: u32) -> Self {
      SpecCoreFuncs::rotr(integer.ucast(), bits).ucast()
    }

    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(integer: Self) -> u32 {
      SpecCoreFuncs::ctpop(integer.ucast())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    #[inline]
    fn ctlz(integer: Self) -> u32 {
      SpecCoreFuncs::ctlz(integer.ucast())
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(integer: Self) -> u32 {
      SpecCoreFuncs::cttz(integer.ucast())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn ctlz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCoreFuncs::ctlz_nonzero(integer.ucast()) }
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn cttz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCoreFuncs::cttz_nonzero(integer.ucast()) }
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `shl $type` instruction
    #[inline]
    unsafe fn unchecked_shl(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCoreFuncs::unchecked_shl(integer.ucast(), bits) }.ucast()
    }

    // -------------------------------------------------------------------------
    // Wrapping Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add $type` instruction
    #[inline]
    fn wrapping_add(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_add(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_sub(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_mul(lhs.ucast(), rhs.ucast()).ucast()
    }
  }
}

specialize! {
  impl const SpecSintFuncs for Int<1|2|4|8|16> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.scmp.i8.$type` intrinsic
    #[inline]
    fn scmp(lhs: Self, rhs: Self) -> Ordering {
      SpecSintFuncs::scmp(lhs.scast(), rhs.scast())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.sadd.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecSintFuncs::overflowing_sadd(lhs.scast(), rhs.scast()).scast()
    }

    // LLVM generates `@llvm.ssub.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecSintFuncs::overflowing_ssub(lhs.scast(), rhs.scast()).scast()
    }

    // LLVM generates `@llvm.smul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecSintFuncs::overflowing_smul(lhs.scast(), rhs.scast()).scast()
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.sadd.sat.$type` intrinsic
    #[inline]
    fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
      SpecSintFuncs::saturating_sadd(lhs.scast(), rhs.scast()).scast()
    }

    // LLVM generates `@llvm.ssub.sat.$type` intrinsic
    #[inline]
    fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
      SpecSintFuncs::saturating_ssub(lhs.scast(), rhs.scast()).scast()
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add nsw $type` instruction
    #[inline]
    unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_sadd(lhs.scast(), rhs.scast()) }.scast()
    }

    // LLVM generates `sub nsw $type` instruction
    #[inline]
    unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_ssub(lhs.scast(), rhs.scast()) }.scast()
    }

    // LLVM generates `mul nsw $type` instruction
    #[inline]
    unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_smul(lhs.scast(), rhs.scast()) }.scast()
    }

    // LLVM generates `sdiv $type` instruction
    #[inline]
    unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_sdiv(lhs.scast(), rhs.scast()) }.scast()
    }

    // LLVM generates `srem $type` instruction
    #[inline]
    unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_srem(lhs.scast(), rhs.scast()) }.scast()
    }

    // LLVM generates `ashr $type` instruction
    #[inline]
    unsafe fn unchecked_ashr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_ashr(integer.scast(), bits) }.scast()
    }
  }
}

specialize! {
  impl const SpecUintFuncs for Int<1|2|4|8|16> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.ucmp.i8.$type` intrinsic
    #[inline]
    fn ucmp(lhs: Self, rhs: Self) -> Ordering {
      SpecUintFuncs::ucmp(lhs.ucast(), rhs.ucast())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.uadd.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecUintFuncs::overflowing_uadd(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `sub $type` instruction
    //
    // Note: `@llvm.usub.with.overflow.$type` is not emitted by rustc:
    //   https://github.com/rust-lang/rust/pull/103299
    #[inline]
    fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecUintFuncs::overflowing_usub(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
      SpecUintFuncs::overflowing_umul(lhs.ucast(), rhs.ucast()).ucast()
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.uadd.sat.$type` intrinsic
    #[inline]
    fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
      SpecUintFuncs::saturating_uadd(lhs.ucast(), rhs.ucast()).ucast()
    }

    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn saturating_usub(lhs: Self, rhs: Self) -> Self {
      SpecUintFuncs::saturating_usub(lhs.ucast(), rhs.ucast()).ucast()
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add nuw $type` instruction
    #[inline]
    unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_uadd(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    // LLVM generates `sub nuw $type` instruction
    #[inline]
    unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_usub(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    // LLVM generates `mul nuw $type` instruction
    #[inline]
    unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_umul(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_udiv(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_urem(lhs.ucast(), rhs.ucast()) }.ucast()
    }

    // LLVM generates `lshr $type` instruction
    #[inline]
    unsafe fn unchecked_lshr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_lshr(integer.ucast(), bits) }.ucast()
    }
  }
}
