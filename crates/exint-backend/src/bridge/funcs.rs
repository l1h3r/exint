use ::core::cmp::Ordering;
use ::core::marker::Copy;

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
use crate::macros::cast;
use crate::types::Integer;
use crate::utils::resize;

// -----------------------------------------------------------------------------
// Type Conversion
// -----------------------------------------------------------------------------

/// Cast between integer types of different sizes.
#[must_use]
#[inline]
#[track_caller]
pub const fn cast<const T: usize, const U: usize, const UINT: bool>(integer: Integer<T>) -> Integer<U> {
  resize::<T, U, UINT>(integer)
}

// -----------------------------------------------------------------------------
// Bitwise Ops
// -----------------------------------------------------------------------------

/// Performs bitwise `AND` (logical AND).
#[must_use]
#[inline]
#[track_caller]
pub const fn band<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  cast!(T, SpecBitwise::and(cast!(S, lhs), cast!(S, rhs)))
}

/// Performs bitwise `OR` (logical inclusive OR).
#[must_use]
#[inline]
#[track_caller]
pub const fn bor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  cast!(T, SpecBitwise::or(cast!(S, lhs), cast!(S, rhs)))
}

/// Performs bitwise `XOR` (logical exclusive OR).
#[must_use]
#[inline]
#[track_caller]
pub const fn bxor<T: Copy, const S: usize>(lhs: T, rhs: T) -> T {
  cast!(T, SpecBitwise::xor(cast!(S, lhs), cast!(S, rhs)))
}

/// Performs bitwise `NOT` (logical negation).
#[must_use]
#[inline]
#[track_caller]
pub const fn bnot<T: Copy, const S: usize>(integer: T) -> T {
  cast!(T, SpecBitwise::not(cast!(S, integer)))
}

// -----------------------------------------------------------------------------
// Comparison
// -----------------------------------------------------------------------------

/// Tests if two integer values are equal.
#[must_use]
#[inline]
#[track_caller]
pub const fn eq<T: Copy, const S: usize>(lhs: T, rhs: T) -> bool {
  SpecCompare::eq(cast!(S, lhs), cast!(S, rhs))
}

/// Returns an ordering between two integer values.
#[must_use]
#[inline]
#[track_caller]
pub const fn cmp<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> Ordering {
  if UINT {
    SpecCompare::ucmp(cast!(S, lhs), cast!(S, rhs))
  } else {
    SpecCompare::scmp(cast!(S, lhs), cast!(S, rhs))
  }
}

// -----------------------------------------------------------------------------
// Bit Conversion
// -----------------------------------------------------------------------------

/// Reverses the bits in an integer type `T`.
#[must_use]
#[inline]
#[track_caller]
pub const fn swap1<T: Copy, const S: usize>(integer: T) -> T {
  cast!(T, SpecConvert::swap1(cast!(S, integer)))
}

/// Reverses the bytes in an integer type `T`.
#[must_use]
#[inline]
#[track_caller]
pub const fn swap8<T: Copy, const S: usize>(integer: T) -> T {
  cast!(T, SpecConvert::swap8(cast!(S, integer)))
}

/// Performs rotate left.
#[must_use]
#[inline]
#[track_caller]
pub const fn rotl<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  cast!(T, SpecConvert::rotl(cast!(S, integer), bits))
}

/// Performs rotate right.
#[must_use]
#[inline]
#[track_caller]
pub const fn rotr<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
  cast!(T, SpecConvert::rotr(cast!(S, integer), bits))
}

// -----------------------------------------------------------------------------
// Bit Inspection
// -----------------------------------------------------------------------------

/// Returns the number of bits set in an integer type `T`.
#[must_use]
#[inline]
#[track_caller]
pub const fn ctpop<T: Copy, const S: usize>(integer: T) -> u32 {
  SpecInspect::ctpop(cast!(S, integer))
}

/// Returns the number of leading unset bits (zeroes) in an integer type `T`.
#[must_use]
#[inline]
#[track_caller]
pub const fn ctlz<T: Copy, const S: usize>(integer: T) -> u32 {
  SpecInspect::ctlz(cast!(S, integer))
}

/// Returns the number of trailing unset bits (zeroes) in an integer type `T`.
#[must_use]
#[inline]
#[track_caller]
pub const fn cttz<T: Copy, const S: usize>(integer: T) -> u32 {
  SpecInspect::cttz(cast!(S, integer))
}

/// Returns the number of leading unset bits (zeroes) in an integer type `T`.
///
/// # Safety
///
/// This results in undefined behavior when given an `integer` with value `0`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn ctlz_nonzero<T: Copy, const S: usize>(integer: T) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { SpecInspect::ctlz_nonzero(cast!(S, integer)) }
}

/// Returns the number of trailing unset bits (zeroes) in an integer type `T`.
///
/// # Safety
///
/// This results in undefined behavior when given an `integer` with value `0`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn cttz_nonzero<T: Copy, const S: usize>(integer: T) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { SpecInspect::cttz_nonzero(cast!(S, integer)) }
}

// -----------------------------------------------------------------------------
// Arithmetic - Overflowing
// -----------------------------------------------------------------------------

/// Performs checked integer addition.
#[must_use]
#[inline]
#[track_caller]
pub const fn overflowing_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> (T, bool) {
  if UINT {
    cast!((T, bool), SpecUadd::oadd(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!((T, bool), SpecSadd::oadd(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs checked integer subtraction.
#[must_use]
#[inline]
#[track_caller]
pub const fn overflowing_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> (T, bool) {
  if UINT {
    cast!((T, bool), SpecUsub::osub(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!((T, bool), SpecSsub::osub(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs checked integer multiplication.
#[must_use]
#[inline]
#[track_caller]
pub const fn overflowing_mul<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> (T, bool) {
  if UINT {
    cast!((T, bool), SpecUmul::omul(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!((T, bool), SpecSmul::omul(cast!(S, lhs), cast!(S, rhs)))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Saturating
// -----------------------------------------------------------------------------

/// Performs `lhs + rhs`, saturating at numeric bounds.
#[must_use]
#[inline]
#[track_caller]
pub const fn saturating_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUadd::sadd(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSadd::sadd(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs `lhs - rhs`, saturating at numeric bounds.
#[must_use]
#[inline]
#[track_caller]
pub const fn saturating_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUsub::ssub(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSsub::ssub(cast!(S, lhs), cast!(S, rhs)))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Unchecked
// -----------------------------------------------------------------------------

/// Performs unchecked integer addition.
///
/// # Safety
///
/// This results in undefined behavior when `lhs + rhs > T::MAX` or `lhs + rhs < T::MIN`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUadd::uadd(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSadd::uadd(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs unchecked integer subtraction.
///
/// # Safety
///
/// This results in undefined behavior when `lhs - rhs > T::MAX` or `lhs - rhs < T::MIN`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUsub::usub(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSsub::usub(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs unchecked integer multiplication.
///
/// # Safety
///
/// This results in undefined behavior when `lhs * rhs > T::MAX` or `lhs * rhs < T::MIN`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_mul<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUmul::umul(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSmul::umul(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs unchecked integer division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_div<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUdiv::udiv(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSdiv::udiv(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Returns the remainder of an unchecked division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_rem<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUdiv::urem(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSdiv::urem(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Performs an unchecked left shift.
///
/// # Safety
///
/// This results in undefined behavior when `bits < 0` or `bits >= N`, where N is the width of T in bits.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_shl<T: Copy, const S: usize, const UINT: bool>(integer: T, bits: u32) -> T {
  cast!(T, SpecShift::shl(cast!(S, integer), bits))
}

/// Performs an unchecked right shift.
///
/// # Safety
///
/// This results in undefined behavior when `bits < 0` or `bits >= N`, where N is the width of T in bits.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_shr<T: Copy, const S: usize, const UINT: bool>(integer: T, bits: u32) -> T {
  if UINT {
    cast!(T, SpecShift::lshr(cast!(S, integer), bits))
  } else {
    cast!(T, SpecShift::ashr(cast!(S, integer), bits))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Wrapping
// -----------------------------------------------------------------------------

/// Returns (a + b) mod 2<sup>N</sup>, where N is the width of T in bits.
#[must_use]
#[inline]
#[track_caller]
pub const fn wrapping_add<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUadd::wadd(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSadd::wadd(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Returns (a - b) mod 2<sup>N</sup>, where N is the width of T in bits.
#[must_use]
#[inline]
#[track_caller]
pub const fn wrapping_sub<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUsub::wsub(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSsub::wsub(cast!(S, lhs), cast!(S, rhs)))
  }
}

/// Returns (a * b) mod 2<sup>N</sup>, where N is the width of T in bits.
#[must_use]
#[inline]
#[track_caller]
pub const fn wrapping_mul<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUmul::wmul(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSmul::wmul(cast!(S, lhs), cast!(S, rhs)))
  }
}

// -----------------------------------------------------------------------------
// Arithmetic - Misc.
// -----------------------------------------------------------------------------

/// Performs an exact division.
///
/// # Safety
///
/// This results in undefined behavior when `lhs % rhs != 0` or `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[must_use]
#[inline]
#[track_caller]
pub const unsafe fn exact_div<T: Copy, const S: usize, const UINT: bool>(lhs: T, rhs: T) -> T {
  if UINT {
    cast!(T, SpecUdiv::ediv(cast!(S, lhs), cast!(S, rhs)))
  } else {
    cast!(T, SpecSdiv::ediv(cast!(S, lhs), cast!(S, rhs)))
  }
}
