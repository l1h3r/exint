use ::core::cmp::Ordering;

use crate::bridge::traits::SpecBitwise;
use crate::bridge::traits::SpecCompare;
use crate::bridge::traits::SpecConvert;
use crate::bridge::traits::SpecInspect;
use crate::bridge::traits::SpecSadd;
use crate::bridge::traits::SpecSdiv;
use crate::bridge::traits::SpecShift;
use crate::bridge::traits::SpecSmul;
use crate::bridge::traits::SpecSsub;
use crate::bridge::traits::SpecUadd;
use crate::bridge::traits::SpecUdiv;
use crate::bridge::traits::SpecUmul;
use crate::bridge::traits::SpecUsub;
use crate::types::Integer;

impl<const S: usize> const SpecBitwise for Integer<S> {
  default fn and(self, _other: Self) -> Self {
    ::core::panic!("SpecBitwise::and")
  }

  default fn or(self, _other: Self) -> Self {
    ::core::panic!("SpecBitwise::or")
  }

  default fn xor(self, _other: Self) -> Self {
    ::core::panic!("SpecBitwise::xor")
  }

  default fn not(self) -> Self {
    ::core::panic!("SpecBitwise::not")
  }
}

impl<const S: usize> const SpecCompare for Integer<S> {
  #[inline]
  default fn eq(self, other: Self) -> bool {
    // SAFETY:
    //   - integer bytes **are not** uninitialized
    //   - integer types **do not** have padding
    unsafe { ::core::intrinsics::raw_eq(&self, &other) }
  }

  default fn ucmp(self, _other: Self) -> Ordering {
    ::core::panic!("SpecCompare::ucmp")
  }

  default fn scmp(self, _other: Self) -> Ordering {
    ::core::panic!("SpecCompare::scmp")
  }
}

impl<const S: usize> const SpecConvert for Integer<S> {
  default fn swap1(self) -> Self {
    ::core::panic!("SpecConvert::swap1")
  }

  default fn swap8(self) -> Self {
    ::core::panic!("SpecConvert::swap8")
  }

  default fn rotl(self, _bits: u32) -> Self {
    ::core::panic!("SpecConvert::rotl")
  }

  default fn rotr(self, _bits: u32) -> Self {
    ::core::panic!("SpecConvert::rotr")
  }
}

impl<const S: usize> const SpecInspect for Integer<S> {
  default fn ctpop(self) -> u32 {
    ::core::panic!("SpecInspect::ctpop")
  }

  default fn ctlz(self) -> u32 {
    ::core::panic!("SpecInspect::ctlz")
  }

  default fn cttz(self) -> u32 {
    ::core::panic!("SpecInspect::cttz")
  }

  default unsafe fn ctlz_nonzero(self) -> u32 {
    ::core::panic!("SpecInspect::ctlz_nonzero")
  }

  default unsafe fn cttz_nonzero(self) -> u32 {
    ::core::panic!("SpecInspect::cttz_nonzero")
  }
}

impl<const S: usize> const SpecShift for Integer<S> {
  default unsafe fn shl(self, _bits: u32) -> Self {
    ::core::panic!("SpecShift::shl")
  }

  default unsafe fn lshr(self, _bits: u32) -> Self {
    ::core::panic!("SpecShift::lshr")
  }

  default unsafe fn ashr(self, _bits: u32) -> Self {
    ::core::panic!("SpecShift::ashr")
  }
}

impl<const S: usize> const SpecUadd for Integer<S> {
  default fn oadd(self, _other: Self) -> (Self, bool) {
    ::core::panic!("SpecUadd::oadd")
  }

  default fn sadd(self, _other: Self) -> Self {
    ::core::panic!("SpecUadd::sadd")
  }

  default fn wadd(self, _other: Self) -> Self {
    ::core::panic!("SpecUadd::wadd")
  }

  default unsafe fn uadd(self, _other: Self) -> Self {
    ::core::panic!("SpecUadd::uadd")
  }
}

impl<const S: usize> const SpecSadd for Integer<S> {
  default fn oadd(self, _other: Self) -> (Self, bool) {
    ::core::panic!("SpecSadd::oadd")
  }

  default fn sadd(self, _other: Self) -> Self {
    ::core::panic!("SpecSadd::sadd")
  }

  default fn wadd(self, _other: Self) -> Self {
    ::core::panic!("SpecSadd::wadd")
  }

  default unsafe fn uadd(self, _other: Self) -> Self {
    ::core::panic!("SpecSadd::uadd")
  }
}

impl<const S: usize> const SpecUsub for Integer<S> {
  default fn osub(self, _other: Self) -> (Self, bool) {
    ::core::panic!("SpecUsub::osub")
  }

  default fn ssub(self, _other: Self) -> Self {
    ::core::panic!("SpecUsub::ssub")
  }

  default fn wsub(self, _other: Self) -> Self {
    ::core::panic!("SpecUsub::wsub")
  }

  default unsafe fn usub(self, _other: Self) -> Self {
    ::core::panic!("SpecUsub::usub")
  }
}

impl<const S: usize> const SpecSsub for Integer<S> {
  default fn osub(self, _other: Self) -> (Self, bool) {
    ::core::panic!("SpecSsub::osub")
  }

  default fn ssub(self, _other: Self) -> Self {
    ::core::panic!("SpecSsub::ssub")
  }

  default fn wsub(self, _other: Self) -> Self {
    ::core::panic!("SpecSsub::wsub")
  }

  default unsafe fn usub(self, _other: Self) -> Self {
    ::core::panic!("SpecSsub::usub")
  }
}

impl<const S: usize> const SpecUmul for Integer<S> {
  default fn omul(self, _other: Self) -> (Self, bool) {
    ::core::panic!("SpecUmul::omul")
  }

  default fn wmul(self, _other: Self) -> Self {
    ::core::panic!("SpecUmul::wmul")
  }

  default unsafe fn umul(self, _other: Self) -> Self {
    ::core::panic!("SpecUmul::umul")
  }
}

impl<const S: usize> const SpecSmul for Integer<S> {
  default fn omul(self, _other: Self) -> (Self, bool) {
    ::core::panic!("SpecSmul::omul")
  }

  default fn wmul(self, _other: Self) -> Self {
    ::core::panic!("SpecSmul::wmul")
  }

  default unsafe fn umul(self, _other: Self) -> Self {
    ::core::panic!("SpecSmul::umul")
  }
}

impl<const S: usize> const SpecUdiv for Integer<S> {
  default unsafe fn udiv(self, _other: Self) -> Self {
    ::core::panic!("SpecUdiv::udiv")
  }

  default unsafe fn urem(self, _other: Self) -> Self {
    ::core::panic!("SpecUdiv::urem")
  }

  default unsafe fn ediv(self, _other: Self) -> Self {
    ::core::panic!("SpecUdiv::ediv")
  }
}

impl<const S: usize> const SpecSdiv for Integer<S> {
  default unsafe fn udiv(self, _other: Self) -> Self {
    ::core::panic!("SpecSdiv::udiv")
  }

  default unsafe fn urem(self, _other: Self) -> Self {
    ::core::panic!("SpecSdiv::urem")
  }

  default unsafe fn ediv(self, _other: Self) -> Self {
    ::core::panic!("SpecSdiv::ediv")
  }
}
