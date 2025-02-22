use ::core::cmp::Ordering;
use ::core::intrinsics;

use crate::macros::specialize;
use crate::traits::Cast;

specialize! {
  impl const SpecBitwise for Integer<1|2|4|8|16> {
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

specialize! {
  impl const SpecCompare for Integer<1|2|4|8|16> {
    // LLVM generates `icmp ult $type` and `icmp ne $type` instructions
    #[inline]
    fn ucmp(self, other: Self) -> Ordering {
      intrinsics::three_way_compare(self.ucast(), other.ucast())
    }

    // LLVM generates `icmp slt $type` and `icmp ne $type` instructions
    #[inline]
    fn scmp(self, other: Self) -> Ordering {
      intrinsics::three_way_compare(self.scast(), other.scast())
    }
  }
}

specialize! {
  impl const SpecConvert for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    #[inline]
    fn swap1(self) -> Self {
      intrinsics::bitreverse(self.ucast()).ucast()
    }

    // LLVM generates `@llvm.bswap.$type` intrinsic
    #[inline]
    fn swap8(self) -> Self {
      intrinsics::bswap(self.ucast()).ucast()
    }

    // LLVM generates `@llvm.fshl.$type` intrinsic
    #[inline]
    fn rotl(self, bits: u32) -> Self {
      intrinsics::rotate_left(self.ucast(), bits).ucast()
    }

    // LLVM generates `@llvm.fshr.$type` intrinsic
    #[inline]
    fn rotr(self, bits: u32) -> Self {
      intrinsics::rotate_right(self.ucast(), bits).ucast()
    }
  }
}

specialize! {
  impl const SpecInspect for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(self) -> u32 {
      intrinsics::ctpop(self.ucast())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    #[inline]
    fn ctlz(self) -> u32 {
      intrinsics::ctlz(self.ucast())
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(self) -> u32 {
      intrinsics::cttz(self.ucast())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn ctlz_nonzero(self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::ctlz_nonzero(self.ucast()) }
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn cttz_nonzero(self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::cttz_nonzero(self.ucast()) }
    }
  }
}

specialize! {
  impl const SpecShift for Integer<1|2|4|8|16> {
    // LLVM generates `shl $type` instruction
    #[inline]
    unsafe fn shl(self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_shl(self.ucast(), bits).ucast() }
    }

    // LLVM generates `lshr $type` instruction
    #[inline]
    unsafe fn lshr(self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_shr(self.ucast(), bits).ucast() }
    }

    // LLVM generates `ashr $type` instruction
    #[inline]
    unsafe fn ashr(self, bits: u32) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_shr(self.scast(), bits).scast() }
    }
  }
}

specialize! {
  impl const SpecUadd for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.uadd.with.overflow.$type` intrinsic
    #[inline]
    fn oadd(self, other: Self) -> (Self, bool) {
      intrinsics::add_with_overflow(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `@llvm.uadd.sat.$type` intrinsic
    #[inline]
    fn sadd(self, other: Self) -> Self {
      intrinsics::saturating_add(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `add $type` instruction
    #[inline]
    fn wadd(self, other: Self) -> Self {
      intrinsics::wrapping_add(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `add nuw $type` instruction
    #[inline]
    unsafe fn uadd(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_add(self.ucast(), other.ucast()).ucast() }
    }
  }
}

specialize! {
  impl const SpecSadd for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.sadd.with.overflow.$type` intrinsic
    #[inline]
    fn oadd(self, other: Self) -> (Self, bool) {
      intrinsics::add_with_overflow(self.scast(), other.scast()).scast()
    }

    // LLVM generates `@llvm.sadd.sat.$type` intrinsic
    #[inline]
    fn sadd(self, other: Self) -> Self {
      intrinsics::saturating_add(self.scast(), other.scast()).scast()
    }

    // LLVM generates `add $type` instruction
    #[inline]
    fn wadd(self, other: Self) -> Self {
      intrinsics::wrapping_add(self.scast(), other.scast()).scast()
    }

    // LLVM generates `add nsw $type` instruction
    #[inline]
    unsafe fn uadd(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_add(self.scast(), other.scast()).scast() }
    }
  }
}

specialize! {
  impl const SpecUsub for Integer<1|2|4|8|16> {
    // LLVM generates `sub $type` instruction
    //
    // TODO: Why doesn't this generate `@llvm.usub.with.overflow.$type` ?
    //
    // Note: Not emitted by rustc so ignore above
    //   https://github.com/rust-lang/rust/blob/8c39ce5b4fb5b61796e5fd8cec56c7b9abd2122b/compiler/rustc_codegen_llvm/src/builder.rs#L371
    #[inline]
    fn osub(self, other: Self) -> (Self, bool) {
      intrinsics::sub_with_overflow(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn ssub(self, other: Self) -> Self {
      intrinsics::saturating_sub(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      intrinsics::wrapping_sub(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `sub nuw $type` instruction
    #[inline]
    unsafe fn usub(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_sub(self.ucast(), other.ucast()).ucast() }
    }
  }
}

specialize! {
  impl const SpecSsub for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.ssub.with.overflow.$type` intrinsic
    #[inline]
    fn osub(self, other: Self) -> (Self, bool) {
      intrinsics::sub_with_overflow(self.scast(), other.scast()).scast()
    }

    // LLVM generates `@llvm.ssub.sat.$type` intrinsic
    #[inline]
    fn ssub(self, other: Self) -> Self {
      intrinsics::saturating_sub(self.scast(), other.scast()).scast()
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      intrinsics::wrapping_sub(self.scast(), other.scast()).scast()
    }

    // LLVM generates `sub nsw $type` instruction
    #[inline]
    unsafe fn usub(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_sub(self.scast(), other.scast()).scast() }
    }
  }
}

specialize! {
  impl const SpecUmul for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn omul(self, other: Self) -> (Self, bool) {
      intrinsics::mul_with_overflow(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wmul(self, other: Self) -> Self {
      intrinsics::wrapping_mul(self.ucast(), other.ucast()).ucast()
    }

    // LLVM generates `mul nuw $type` instruction
    #[inline]
    unsafe fn umul(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_mul(self.ucast(), other.ucast()).ucast() }
    }
  }
}

specialize! {
  impl const SpecSmul for Integer<1|2|4|8|16> {
    // LLVM generates `@llvm.smul.with.overflow.$type` intrinsic
    #[inline]
    fn omul(self, other: Self) -> (Self, bool) {
      intrinsics::mul_with_overflow(self.scast(), other.scast()).scast()
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wmul(self, other: Self) -> Self {
      intrinsics::wrapping_mul(self.scast(), other.scast()).scast()
    }

    // LLVM generates `mul nsw $type` instruction
    #[inline]
    unsafe fn umul(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_mul(self.scast(), other.scast()).scast() }
    }
  }
}

specialize! {
  impl const SpecUdiv for Integer<1|2|4|8|16> {
    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn udiv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_div(self.ucast(), other.ucast()).ucast() }
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn urem(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_rem(self.ucast(), other.ucast()).ucast() }
    }

    // LLVM generates `udiv exact $type` instruction
    #[inline]
    unsafe fn ediv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::exact_div(self.ucast(), other.ucast()).ucast() }
    }
  }
}

specialize! {
  impl const SpecSdiv for Integer<1|2|4|8|16> {
    // LLVM generates `sdiv $type` instruction
    #[inline]
    unsafe fn udiv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_div(self.scast(), other.scast()).scast() }
    }

    // LLVM generates `srem $type` instruction
    #[inline]
    unsafe fn urem(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_rem(self.scast(), other.scast()).scast() }
    }

    // LLVM generates `sdiv exact $type` instruction
    #[inline]
    unsafe fn ediv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::exact_div(self.scast(), other.scast()).scast() }
    }
  }
}
