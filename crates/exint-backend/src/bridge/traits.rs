//! Supporting traits for integer operations, for internal use only.

use ::core::cmp::Ordering;
use ::core::marker::Sized;

/// Supporting trait for bitwise operations.
#[const_trait]
pub(crate) trait SpecBitwise {
  fn and(self, other: Self) -> Self;
  fn or(self, other: Self) -> Self;
  fn xor(self, other: Self) -> Self;
  fn not(self) -> Self;
}

/// Supporting trait for comparison operations.
#[const_trait]
pub(crate) trait SpecCompare {
  fn eq(self, other: Self) -> bool;
  fn ucmp(self, other: Self) -> Ordering;
  fn scmp(self, other: Self) -> Ordering;
}

/// Supporting trait for conversion operations.
#[const_trait]
pub(crate) trait SpecConvert {
  fn swap1(self) -> Self;
  fn swap8(self) -> Self;
  fn rotl(self, bits: u32) -> Self;
  fn rotr(self, bits: u32) -> Self;
}

/// Supporting trait for inspection operations.
#[const_trait]
pub(crate) trait SpecInspect {
  fn ctpop(self) -> u32;
  fn ctlz(self) -> u32;
  fn cttz(self) -> u32;
  unsafe fn ctlz_nonzero(self) -> u32;
  unsafe fn cttz_nonzero(self) -> u32;
}

/// Supporting trait for bitwise shift operations.
#[const_trait]
pub(crate) trait SpecShift {
  unsafe fn shl(self, bits: u32) -> Self;
  unsafe fn lshr(self, bits: u32) -> Self;
  unsafe fn ashr(self, bits: u32) -> Self;
}

/// Supporting trait for unsigned addition operations.
#[const_trait]
pub(crate) trait SpecUadd: Sized {
  fn oadd(self, other: Self) -> (Self, bool);
  fn sadd(self, other: Self) -> Self;
  fn wadd(self, other: Self) -> Self;
  unsafe fn uadd(self, other: Self) -> Self;
}

/// Supporting trait for signed addition operations.
#[const_trait]
pub(crate) trait SpecSadd: SpecUadd {
  fn oadd(self, other: Self) -> (Self, bool);
  fn sadd(self, other: Self) -> Self;
  fn wadd(self, other: Self) -> Self;
  unsafe fn uadd(self, other: Self) -> Self;
}

/// Supporting trait for unsigned subtraction operations.
#[const_trait]
pub(crate) trait SpecUsub: Sized {
  fn osub(self, other: Self) -> (Self, bool);
  fn ssub(self, other: Self) -> Self;
  fn wsub(self, other: Self) -> Self;
  unsafe fn usub(self, other: Self) -> Self;
}

/// Supporting trait for signed subtraction operations.
#[const_trait]
pub(crate) trait SpecSsub: SpecUsub {
  fn osub(self, other: Self) -> (Self, bool);
  fn ssub(self, other: Self) -> Self;
  fn wsub(self, other: Self) -> Self;
  unsafe fn usub(self, other: Self) -> Self;
}

/// Supporting trait for unsigned multiplication operations.
#[const_trait]
pub(crate) trait SpecUmul: Sized {
  fn omul(self, other: Self) -> (Self, bool);
  fn wmul(self, other: Self) -> Self;
  unsafe fn umul(self, other: Self) -> Self;
}

/// Supporting trait for signed multiplication operations.
#[const_trait]
pub(crate) trait SpecSmul: SpecUmul {
  fn omul(self, other: Self) -> (Self, bool);
  fn wmul(self, other: Self) -> Self;
  unsafe fn umul(self, other: Self) -> Self;
}

/// Supporting trait for unsigned division operations.
#[const_trait]
pub(crate) trait SpecUdiv {
  unsafe fn udiv(self, other: Self) -> Self;
  unsafe fn urem(self, other: Self) -> Self;
  unsafe fn ediv(self, other: Self) -> Self;
}

/// Supporting trait for signed division operations.
#[const_trait]
pub(crate) trait SpecSdiv: SpecUdiv {
  unsafe fn udiv(self, other: Self) -> Self;
  unsafe fn urem(self, other: Self) -> Self;
  unsafe fn ediv(self, other: Self) -> Self;
}
