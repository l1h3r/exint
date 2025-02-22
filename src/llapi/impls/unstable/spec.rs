use ::core::cmp::Ordering;

use crate::llapi::impls::unstable::SpecCoreFuncs;
use crate::llapi::impls::unstable::SpecSintFuncs;
use crate::llapi::impls::unstable::SpecUintFuncs;
use crate::llapi::macros::specialize;
use crate::llapi::traits::Consts;
use crate::llapi::traits::Exts;
use crate::llapi::traits::Trunc;

specialize! {
  impl const SpecCoreFuncs for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------

    // LLVM generates `and $type` instruction
    #[inline]
    fn band(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::band(lhs.zext(), rhs.zext()).trunc()
    }

    // LLVM generates `or $type` instruction
    #[inline]
    fn bor(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::bor(lhs.zext(), rhs.zext()).trunc()
    }

    // LLVM generates `xor $type` instruction
    #[inline]
    fn bxor(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::bxor(lhs.zext(), rhs.zext()).trunc()
    }

    // LLVM generates `xor $type .. -1` instruction
    #[inline]
    fn bnot(integer: Self) -> Self {
      (SpecCoreFuncs::bnot(integer.zext()) & Self::UMAX.zext()).trunc()
    }

    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp eq $type` instruction
    #[inline]
    fn eq(lhs: Self, rhs: Self) -> bool {
      SpecCoreFuncs::eq(lhs.zext(), rhs.zext())
    }

    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    //
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

    // LLVM generates `@llvm.bswap.$type` intrinsic
    //
    // Note: This intrinsic is only supported when `N % 16 == 0`, where N is the
    //       width of `Self` in bits.
    #[inline]
    fn swap8(integer: Self) -> Self {
      (SpecCoreFuncs::swap8(integer.zext() << Self::UDIFF)).trunc()
    }

    // LLVM generates `@llvm.fshl.$type` intrinsic
    #[inline]
    fn rotl(_integer: Self, _bits: u32) -> Self {
      ::core::panic!("rotl")
    }

    // LLVM generates `@llvm.fshr.$type` intrinsic
    #[inline]
    fn rotr(_integer: Self, _bits: u32) -> Self {
      ::core::panic!("rotr")
    }

    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(integer: Self) -> u32 {
      SpecCoreFuncs::ctpop(integer.zext())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    //
    // Note: LLVM only recognizes this pattern when increasing the loop unroll
    //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
    #[inline]
    fn ctlz(integer: Self) -> u32 {
      SpecCoreFuncs::cttz(SpecCoreFuncs::swap1(integer))
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(integer: Self) -> u32 {
      if integer.zext() == 0 {
        return Self::BITS;
      }

      SpecCoreFuncs::cttz(integer.zext())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn ctlz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCoreFuncs::cttz_nonzero(SpecCoreFuncs::swap1(integer)) }
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn cttz_nonzero(integer: Self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCoreFuncs::cttz_nonzero(integer.zext()) }
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `shl $builtin` instruction
    //
    // TODO: Figure out how to get `shl $type`
    #[inline]
    unsafe fn unchecked_shl(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecCoreFuncs::unchecked_shl(integer.zext(), bits) }.trunc()
    }

    // -------------------------------------------------------------------------
    // Wrapping Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add $type` instruction
    #[inline]
    fn wrapping_add(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let out: <Self as Exts>::Uint = SpecCoreFuncs::wrapping_add(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let out: <Self as Exts>::Uint = SpecCoreFuncs::wrapping_sub(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();
      let out: <Self as Exts>::Uint = SpecCoreFuncs::wrapping_mul(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }
  }
}

specialize! {
  impl const SpecSintFuncs for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp slt $type` and `icmp ne $type` instructions
    #[inline]
    fn scmp(lhs: Self, rhs: Self) -> Ordering {
      SpecSintFuncs::scmp(lhs.sext(), rhs.sext())
    }

    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.sadd.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_sadd(_lhs: Self, _rhs: Self) -> (Self, bool) {
      ::core::panic!("overflowing_sadd")
    }

    // LLVM generates `@llvm.ssub.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_ssub(_lhs: Self, _rhs: Self) -> (Self, bool) {
      ::core::panic!("overflowing_ssub")
    }

    // LLVM generates `@llvm.smul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_smul(_lhs: Self, _rhs: Self) -> (Self, bool) {
      ::core::panic!("overflowing_smul")
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.sadd.sat.$type` intrinsic
    #[inline]
    fn saturating_sadd(_lhs: Self, _rhs: Self) -> Self {
      ::core::panic!("saturating_sadd")
    }

    // LLVM generates `@llvm.ssub.sat.$type` intrinsic
    #[inline]
    fn saturating_ssub(_lhs: Self, _rhs: Self) -> Self {
      ::core::panic!("saturating_ssub")
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add $type` instruction
    //
    // TODO: Figure out how to get `nsw` keyword
    #[inline]
    unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_add(lhs, rhs)
    }

    // LLVM generates `sub $type` instruction
    //
    // TODO: Figure out how to get `nsw` keyword
    #[inline]
    unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_sub(lhs, rhs)
    }

    // LLVM generates `mul $type` instruction
    //
    // TODO: Figure out how to get `nsw` keyword
    #[inline]
    unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_mul(lhs, rhs)
    }

    // LLVM generates `sdiv $builtin` instruction
    //
    // TODO: Figure out how to get `sdiv $type`
    //
    // Note: LLVM does not actually seem to support narrowing sdiv
    #[inline]
    unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_sdiv(lhs.sext(), rhs.sext()) }.trunc()
    }

    // LLVM generates `srem $builtin` instruction
    //
    // TODO: Figure out how to get `srem $type`
    //
    // Note: LLVM does not actually seem to support narrowing srem
    #[inline]
    unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_srem(lhs.sext(), rhs.sext()) }.trunc()
    }

    // LLVM generates `ashr $builtin` instruction
    //
    // TODO: Figure out how to get `ashr $type`
    #[inline]
    unsafe fn unchecked_ashr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecSintFuncs::unchecked_ashr(integer.sext(), bits) }.trunc()
    }
  }
}

specialize! {
  impl const SpecUintFuncs for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------

    // LLVM generates `icmp ult $type` and `icmp ne $type` instructions
    #[inline]
    fn ucmp(lhs: Self, rhs: Self) -> Ordering {
      SpecUintFuncs::ucmp(lhs.zext(), rhs.zext())
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
      let out: Self = SpecCoreFuncs::wrapping_add(lhs, rhs);
      let cmp: bool = SpecUintFuncs::ucmp(out, lhs).is_lt();

      (out, cmp)
    }

    // LLVM generates `sub $type` and `icmp ult $type` instructions
    //
    // Note: This appears to be the recommended pattern:
    //   https://github.com/rust-lang/rust/pull/103299
    #[inline]
    fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
      let out: Self = SpecCoreFuncs::wrapping_sub(lhs, rhs);
      let cmp: bool = SpecUintFuncs::ucmp(lhs, rhs).is_lt();

      (out, cmp)
    }

    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
      let lhs: <Self as Exts>::Uint = lhs.zext();
      let rhs: <Self as Exts>::Uint = rhs.zext();

      // SAFETY: Multiplication cannot overflow the next power-of-two size.
      let out: <Self as Exts>::Uint = unsafe {
        SpecUintFuncs::unchecked_umul(lhs, rhs)
      };

      (out.trunc(), out > Self::UMAX.zext())
    }

    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------

    // LLVM generates `@llvm.uadd.sat.$type` intrinsic
    #[inline]
    fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
      let out: Self = SpecCoreFuncs::wrapping_add(lhs, rhs);
      let cmp: bool = SpecUintFuncs::ucmp(out, lhs).is_lt();

      if cmp {
        return Self::UMAX;
      }

      out
    }

    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn saturating_usub(lhs: Self, rhs: Self) -> Self {
      if SpecUintFuncs::ucmp(lhs, rhs).is_lt() {
        return Self::UMIN;
      }

      SpecCoreFuncs::wrapping_sub(lhs, rhs)
    }

    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------

    // LLVM generates `add $type` instruction
    //
    // TODO: Figure out how to get `nuw` keyword
    #[inline]
    unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_add(lhs, rhs)
    }

    // LLVM generates `sub $type` instruction
    //
    // TODO: Figure out how to get `nuw` keyword
    #[inline]
    unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_sub(lhs, rhs)
    }

    // LLVM generates `mul $type` instruction
    //
    // TODO: Figure out how to get `nuw` keyword
    #[inline]
    unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
      SpecCoreFuncs::wrapping_mul(lhs, rhs)
    }

    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_udiv(lhs.zext(), rhs.zext()) }.trunc()
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_urem(lhs.zext(), rhs.zext()) }.trunc()
    }

    // LLVM generates `lshr $builtin` instruction
    //
    // TODO: Figure out how to get `lshr $type`
    #[inline]
    unsafe fn unchecked_lshr(integer: Self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecUintFuncs::unchecked_lshr(integer.zext(), bits) }.trunc()
    }
  }
}
