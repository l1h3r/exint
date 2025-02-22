use ::core::cmp::Ordering;
use ::core::intrinsics;

use crate::bridge::traits::SpecCompare;
use crate::bridge::traits::SpecConvert;
use crate::bridge::traits::SpecSadd;
use crate::bridge::traits::SpecSdiv;
use crate::bridge::traits::SpecSmul;
use crate::bridge::traits::SpecSsub;
use crate::bridge::traits::SpecUadd;
use crate::bridge::traits::SpecUmul;
use crate::bridge::traits::SpecUsub;
use crate::macros::cfor;
use crate::macros::specialize;
use crate::traits::Consts;
use crate::traits::Exts;
use crate::traits::Trunc;

specialize! {
  impl const SpecBitwise for Integer<3|5|6|7|9|10|11|12|13|14|15> {
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
      (!self.zext() & Self::UMAX.zext()).trunc()
    }
  }
}

specialize! {
  impl const SpecCompare for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `icmp ult $type` and `icmp ne $type` instructions
    #[inline]
    fn ucmp(self, other: Self) -> Ordering {
      intrinsics::three_way_compare(self.zext(), other.zext())
    }

    // LLVM generates `icmp slt $type` and `icmp ne $type` instructions
    #[inline]
    fn scmp(self, other: Self) -> Ordering {
      intrinsics::three_way_compare(self.sext(), other.sext())
    }
  }
}

specialize! {
  impl const SpecConvert for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `@llvm.bitreverse.$type` intrinsic
    //
    // Note: LLVM only recognizes this pattern when increasing the loop unroll
    //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
    #[inline]
    fn swap1(self) -> Self {
      let mut input: <Self as Exts>::Uint = self.zext();
      let mut value: <Self as Exts>::Uint = 0;

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
      intrinsics::bswap(self.zext() << Self::UDIFF).trunc()
    }

    #[inline]
    fn rotl(self, _bits: u32) -> Self {
      ::core::panic!("SpecConvert::rotl")
    }

    #[inline]
    fn rotr(self, _bits: u32) -> Self {
      ::core::panic!("SpecConvert::rotr")
    }
  }
}

specialize! {
  impl const SpecInspect for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(self) -> u32 {
      intrinsics::ctpop(self.zext())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    #[inline]
    fn ctlz(self) -> u32 {
      SpecConvert::swap1(self).cttz()
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(self) -> u32 {
      if self.zext() == 0 {
        return Self::BITS;
      }

      intrinsics::cttz(self.zext())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn ctlz_nonzero(self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { SpecConvert::swap1(self).cttz_nonzero() }
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn cttz_nonzero(self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::cttz_nonzero(self.zext()) }
    }
  }
}

specialize! {
  impl const SpecShift for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    unsafe fn shl(self, _bits: u32) -> Self {
      ::core::panic!("SpecShift::shl")
    }

    #[inline]
    unsafe fn lshr(self, _bits: u32) -> Self {
      ::core::panic!("SpecShift::lshr")
    }

    #[inline]
    unsafe fn ashr(self, _bits: u32) -> Self {
      ::core::panic!("SpecShift::ashr")
    }
  }
}

specialize! {
  impl const SpecUadd for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    fn oadd(self, _other: Self) -> (Self, bool) {
      ::core::panic!("SpecUadd::oadd")
    }

    // LLVM generates `@llvm.uadd.sat.$type` intrinsic
    #[inline]
    fn sadd(self, other: Self) -> Self {
      let out: Self = SpecUadd::wadd(self, other);

      if SpecCompare::ucmp(out, self).is_lt() {
        return Self::UMAX;
      }

      out
    }

    // LLVM generates `add $type` instruction
    #[inline]
    fn wadd(self, other: Self) -> Self {
      let lhs: <Self as Exts>::Uint = self.zext();
      let rhs: <Self as Exts>::Uint = other.zext();

      // SAFETY: Addition cannot overflow the next power-of-two size.
      let out: <Self as Exts>::Uint = unsafe {
        intrinsics::unchecked_add(lhs, rhs)
      };

      (out & Self::UMAX.zext()).trunc()
    }

    // LLVM generates `add $type` instruction
    //
    // TODO: Figure out `nuw` keyword
    #[inline]
    unsafe fn uadd(self, other: Self) -> Self {
      SpecUadd::wadd(self, other)
    }
  }
}

specialize! {
  impl const SpecSadd for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    fn oadd(self, _other: Self) -> (Self, bool) {
      ::core::panic!("SpecSadd::oadd")
    }

    #[inline]
    fn sadd(self, _other: Self) -> Self {
      ::core::panic!("SpecSadd::sadd")
    }

    // LLVM generates `add $type` instruction
    #[inline]
    fn wadd(self, other: Self) -> Self {
      SpecUadd::wadd(self, other)
    }

    // LLVM generates `add $type` instruction
    //
    // TODO: Figure out `nsw` keyword
    #[inline]
    unsafe fn uadd(self, other: Self) -> Self {
      SpecSadd::wadd(self, other)
    }
  }
}

specialize! {
  impl const SpecUsub for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    fn osub(self, _other: Self) -> (Self, bool) {
      ::core::panic!("SpecUsub::osub")
    }

    // LLVM generates `@llvm.usub.sat.$type` intrinsic
    #[inline]
    fn ssub(self, other: Self) -> Self {
      if SpecCompare::ucmp(self, other).is_lt() {
        return Self::UMIN;
      }

      SpecUsub::wsub(self, other)
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      let lhs: <Self as Exts>::Uint = self.zext();
      let rhs: <Self as Exts>::Uint = other.zext();
      let out: <Self as Exts>::Uint = intrinsics::wrapping_sub(lhs, rhs);

      (out & Self::UMAX.zext()).trunc()
    }

    // LLVM generates `sub $type` instruction
    //
    // TODO: Figure out `nuw` keyword
    #[inline]
    unsafe fn usub(self, other: Self) -> Self {
      SpecUsub::wsub(self, other)
    }
  }
}

specialize! {
  impl const SpecSsub for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    fn osub(self, _other: Self) -> (Self, bool) {
      ::core::panic!("SpecSsub::osub")
    }

    #[inline]
    fn ssub(self, _other: Self) -> Self {
      ::core::panic!("SpecSsub::ssub")
    }

    // LLVM generates `sub $type` instruction
    #[inline]
    fn wsub(self, other: Self) -> Self {
      SpecUsub::wsub(self, other)
    }

    // LLVM generates `sub $type` instruction
    //
    // TODO: Figure out `nsw` keyword
    #[inline]
    unsafe fn usub(self, other: Self) -> Self {
      SpecSsub::wsub(self, other)
    }
  }
}

specialize! {
  impl const SpecUmul for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
    #[inline]
    fn omul(self, other: Self) -> (Self, bool) {
      let lhs: <Self as Exts>::Uint = self.zext();
      let rhs: <Self as Exts>::Uint = other.zext();

      // SAFETY: Multiplication cannot overflow the next power-of-two size.
      let out: <Self as Exts>::Uint = unsafe {
        intrinsics::unchecked_mul(lhs, rhs)
      };

      (out.trunc(), out > Self::UMAX.zext())
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wmul(self, other: Self) -> Self {
      let lhs: <Self as Exts>::Uint = self.zext();
      let rhs: <Self as Exts>::Uint = other.zext();

      // SAFETY: Multiplication cannot overflow the next power-of-two size.
      let out: <Self as Exts>::Uint = unsafe {
        intrinsics::unchecked_mul(lhs, rhs)
      };

      (out & Self::UMAX.zext()).trunc()
    }

    // LLVM generates `mul $type` instruction
    //
    // TODO: Figure out `nuw` keyword
    #[inline]
    unsafe fn umul(self, other: Self) -> Self {
      SpecUmul::wmul(self, other)
    }
  }
}

specialize! {
  impl const SpecSmul for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    fn omul(self, _other: Self) -> (Self, bool) {
      ::core::panic!("SpecSmul::omul")
    }

    // LLVM generates `mul $type` instruction
    #[inline]
    fn wmul(self, other: Self) -> Self {
      SpecUmul::wmul(self, other)
    }

    // LLVM generates `mul $type` instruction
    //
    // TODO: Figure out `nsw` keyword
    #[inline]
    unsafe fn umul(self, other: Self) -> Self {
      SpecSmul::wmul(self, other)
    }
  }
}

specialize! {
  impl const SpecUdiv for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `udiv $type` instruction
    #[inline]
    unsafe fn udiv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_div(self.zext(), other.zext()).trunc() }
    }

    // LLVM generates `urem $type` instruction
    #[inline]
    unsafe fn urem(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::unchecked_rem(self.zext(), other.zext()).trunc() }
    }

    // LLVM generates `udiv $type` instruction
    //
    // TODO: Figure out `exact` keyword
    #[inline]
    unsafe fn ediv(self, other: Self) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { intrinsics::exact_div(self.zext(), other.zext()).trunc() }
    }
  }
}

specialize! {
  impl const SpecSdiv for Integer<3|5|6|7|9|10|11|12|13|14|15> {
    #[inline]
    unsafe fn udiv(self, _other: Self) -> Self {
      ::core::panic!("SpecSdiv::udiv")
    }

    #[inline]
    unsafe fn urem(self, _other: Self) -> Self {
      ::core::panic!("SpecSdiv::urem")
    }

    #[inline]
    unsafe fn ediv(self, _other: Self) -> Self {
      ::core::panic!("SpecSdiv::ediv")
    }
  }
}
