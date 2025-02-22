use ::core::cmp::Ordering;

use crate::bridge::traits::BaseBitwise;
use crate::bridge::traits::BaseCompare;
use crate::bridge::traits::BaseConvert;
use crate::bridge::traits::BaseInspect;
use crate::bridge::traits::BaseSadd;
use crate::bridge::traits::BaseSdiv;
use crate::bridge::traits::BaseShift;
use crate::bridge::traits::BaseSmul;
use crate::bridge::traits::BaseSsub;
use crate::bridge::traits::BaseUadd;
use crate::bridge::traits::BaseUdiv;
use crate::bridge::traits::BaseUmul;
use crate::bridge::traits::BaseUsub;
use crate::types::Integer;

impl<const S: usize> const BaseBitwise for Integer<S> {
  default fn and(self, _other: Self) -> Self {
    ::core::panic!("BaseBitwise::and")
  }

  default fn or(self, _other: Self) -> Self {
    ::core::panic!("BaseBitwise::or")
  }

  default fn xor(self, _other: Self) -> Self {
    ::core::panic!("BaseBitwise::xor")
  }

  default fn not(self) -> Self {
    ::core::panic!("BaseBitwise::not")
  }
}

impl<const S: usize> const BaseCompare for Integer<S> {
  #[inline]
  default fn eq(self, other: Self) -> bool {
    // SAFETY:
    //   - integer bytes **are not** uninitialized
    //   - integer types **do not** have padding
    unsafe { ::core::intrinsics::raw_eq(&self, &other) }
  }

  default fn ucmp(self, _other: Self) -> Ordering {
    ::core::panic!("BaseCompare::ucmp")
  }

  default fn scmp(self, _other: Self) -> Ordering {
    ::core::panic!("BaseCompare::scmp")
  }
}

impl<const S: usize> const BaseConvert for Integer<S> {
  default fn swap1(self) -> Self {
    ::core::panic!("BaseConvert::swap1")
  }

  default fn swap8(self) -> Self {
    ::core::panic!("BaseConvert::swap8")
  }

  default fn rotl(self, _bits: u32) -> Self {
    ::core::panic!("BaseConvert::rotl")
  }

  default fn rotr(self, _bits: u32) -> Self {
    ::core::panic!("BaseConvert::rotr")
  }
}

impl<const S: usize> const BaseInspect for Integer<S> {
  default fn ctpop(self) -> u32 {
    ::core::panic!("BaseInspect::ctpop")
  }

  default fn ctlz(self) -> u32 {
    ::core::panic!("BaseInspect::ctlz")
  }

  default fn cttz(self) -> u32 {
    ::core::panic!("BaseInspect::cttz")
  }

  default unsafe fn ctlz_nonzero(self) -> u32 {
    ::core::panic!("BaseInspect::ctlz_nonzero")
  }

  default unsafe fn cttz_nonzero(self) -> u32 {
    ::core::panic!("BaseInspect::cttz_nonzero")
  }
}

impl<const S: usize> const BaseShift for Integer<S> {
  default unsafe fn shl(self, _bits: u32) -> Self {
    ::core::panic!("BaseShift::shl")
  }

  default unsafe fn lshr(self, _bits: u32) -> Self {
    ::core::panic!("BaseShift::lshr")
  }

  default unsafe fn ashr(self, _bits: u32) -> Self {
    ::core::panic!("BaseShift::ashr")
  }
}

impl<const S: usize> const BaseUadd for Integer<S> {
  default fn oadd(self, _other: Self) -> (Self, bool) {
    ::core::panic!("BaseUadd::oadd")
  }

  default fn sadd(self, _other: Self) -> Self {
    ::core::panic!("BaseUadd::sadd")
  }

  default fn wadd(self, _other: Self) -> Self {
    ::core::panic!("BaseUadd::wadd")
  }

  default unsafe fn uadd(self, _other: Self) -> Self {
    ::core::panic!("BaseUadd::uadd")
  }
}

impl<const S: usize> const BaseSadd for Integer<S> {
  default fn oadd(self, _other: Self) -> (Self, bool) {
    ::core::panic!("BaseSadd::oadd")
  }

  default fn sadd(self, _other: Self) -> Self {
    ::core::panic!("BaseSadd::sadd")
  }

  default fn wadd(self, _other: Self) -> Self {
    ::core::panic!("BaseSadd::wadd")
  }

  default unsafe fn uadd(self, _other: Self) -> Self {
    ::core::panic!("BaseSadd::uadd")
  }
}

impl<const S: usize> const BaseUsub for Integer<S> {
  default fn osub(self, _other: Self) -> (Self, bool) {
    ::core::panic!("BaseUsub::osub")
  }

  default fn ssub(self, _other: Self) -> Self {
    ::core::panic!("BaseUsub::ssub")
  }

  default fn wsub(self, _other: Self) -> Self {
    ::core::panic!("BaseUsub::wsub")
  }

  default unsafe fn usub(self, _other: Self) -> Self {
    ::core::panic!("BaseUsub::usub")
  }
}

impl<const S: usize> const BaseSsub for Integer<S> {
  default fn osub(self, _other: Self) -> (Self, bool) {
    ::core::panic!("BaseSsub::osub")
  }

  default fn ssub(self, _other: Self) -> Self {
    ::core::panic!("BaseSsub::ssub")
  }

  default fn wsub(self, _other: Self) -> Self {
    ::core::panic!("BaseSsub::wsub")
  }

  default unsafe fn usub(self, _other: Self) -> Self {
    ::core::panic!("BaseSsub::usub")
  }
}

impl<const S: usize> const BaseUmul for Integer<S> {
  default fn omul(self, _other: Self) -> (Self, bool) {
    ::core::panic!("BaseUmul::omul")
  }

  default fn wmul(self, _other: Self) -> Self {
    ::core::panic!("BaseUmul::wmul")
  }

  default unsafe fn umul(self, _other: Self) -> Self {
    ::core::panic!("BaseUmul::umul")
  }
}

impl<const S: usize> const BaseSmul for Integer<S> {
  default fn omul(self, _other: Self) -> (Self, bool) {
    ::core::panic!("BaseSmul::omul")
  }

  default fn wmul(self, _other: Self) -> Self {
    ::core::panic!("BaseSmul::wmul")
  }

  default unsafe fn umul(self, _other: Self) -> Self {
    ::core::panic!("BaseSmul::umul")
  }
}

impl<const S: usize> const BaseUdiv for Integer<S> {
  default unsafe fn udiv(self, _other: Self) -> Self {
    ::core::panic!("BaseUdiv::udiv")
  }

  default unsafe fn urem(self, _other: Self) -> Self {
    ::core::panic!("BaseUdiv::urem")
  }

  default unsafe fn ediv(self, _other: Self) -> Self {
    ::core::panic!("BaseUdiv::ediv")
  }
}

impl<const S: usize> const BaseSdiv for Integer<S> {
  default unsafe fn udiv(self, _other: Self) -> Self {
    ::core::panic!("BaseSdiv::udiv")
  }

  default unsafe fn urem(self, _other: Self) -> Self {
    ::core::panic!("BaseSdiv::urem")
  }

  default unsafe fn ediv(self, _other: Self) -> Self {
    ::core::panic!("BaseSdiv::ediv")
  }
}
