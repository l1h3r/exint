#![expect(unused_unsafe, reason = "Happens in macro-generated code")]

use ::core::cmp::Ordering;

use crate::llapi::macros::cast;
use crate::llapi::macros::maybe_intrinsic;
use crate::llapi::utils::resize_bytes;
use crate::utils::Uint;

pub(crate) mod api {
  #[cfg(not(feature = "const_trait_impl"))]
  pub(crate) use crate::llapi::impls::fallback::*;
  #[cfg(feature = "const_trait_impl")]
  pub(crate) use crate::llapi::impls::unstable::*;
}

// -----------------------------------------------------------------------------
// Compiler Hints
// -----------------------------------------------------------------------------

/// A hint to the compiler that the given `condition` is likely to be `false`.
#[inline]
#[track_caller]
pub(crate) const fn unlikely(condition: bool) -> bool {
  maybe_intrinsic! {
    @enabled => {
      ::core::intrinsics::unlikely(condition)
    }
    @default => {
      #[cold]
      const fn cold() {}

      if condition {
        cold();
        true
      } else {
        false
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Conversion Operations
// -----------------------------------------------------------------------------

/// Cast between integer types of different sizes.
///
/// This results in one of the following operations:
///
///  * The high order bits of `N` are truncated and returned as `M`.
///  * `N` is extended to `M` and the high order bits are filled with zero bits.
///  * `N` is extended to `M` and the high order bits are filled with the source sign bit.
#[inline]
#[track_caller]
pub(crate) const fn cast_bytes<T: Uint, const N: usize, const M: usize>(bytes: [u8; N]) -> [u8; M] {
  resize_bytes::<T, N, M>(bytes)
}

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

/// Performs bitwise logical `AND`.
#[inline]
#[track_caller]
pub(crate) const fn band<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::band(cast!(N lhs), cast!(N rhs)))
}

/// Performs bitwise logical inclusive `OR`.
#[inline]
#[track_caller]
pub(crate) const fn bor<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::bor(cast!(N lhs), cast!(N rhs)))
}

/// Performs bitwise logical exclusive `OR`.
#[inline]
#[track_caller]
pub(crate) const fn bxor<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::bxor(cast!(N lhs), cast!(N rhs)))
}

/// Performs bitwise logical negation.
#[inline]
#[track_caller]
pub(crate) const fn bnot<T: Uint, const N: usize>(integer: T) -> T {
  cast!(T api::bnot(cast!(N integer)))
}

/// Performs bitwise logical inclusive `OR` with values having no common bits.
///
/// # Safety
///
/// This results in undefined behavior when `(lhs & rhs) != 0`.
#[cfg(feature = "disjoint_bitor")]
#[inline]
#[track_caller]
pub(crate) const unsafe fn disjoint_bor<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::disjoint_bor(cast!(N lhs), cast!(N rhs)))
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

/// Tests if two integer values are equal.
#[inline]
#[track_caller]
pub(crate) const fn eq<T: Uint, const N: usize>(lhs: T, rhs: T) -> bool {
  api::eq(cast!(N lhs), cast!(N rhs))
}

/// Returns an ordering between two unsigned integer values.
#[inline]
#[track_caller]
pub(crate) const fn ucmp<T: Uint, const N: usize>(lhs: T, rhs: T) -> Ordering {
  api::ucmp(cast!(N lhs), cast!(N rhs))
}

/// Returns an ordering between two signed integer values.
#[inline]
#[track_caller]
pub(crate) const fn scmp<T: Uint, const N: usize>(lhs: T, rhs: T) -> Ordering {
  api::scmp(cast!(N lhs), cast!(N rhs))
}

// -----------------------------------------------------------------------------
// Bit Conversion Operation
// -----------------------------------------------------------------------------

/// Reverses the bits in an integer type `T`.
#[inline]
#[track_caller]
pub(crate) const fn swap1<T: Uint, const N: usize>(integer: T) -> T {
  cast!(T api::swap1(cast!(N integer)))
}

/// Reverses the bytes in an integer type `T`.
#[inline]
#[track_caller]
pub(crate) const fn swap8<T: Uint, const N: usize>(integer: T) -> T {
  cast!(T api::swap8(cast!(N integer)))
}

/// Performs rotate left.
#[inline]
#[track_caller]
pub(crate) const fn rotl<T: Uint, const N: usize>(integer: T, bits: u32) -> T {
  cast!(T api::rotl(cast!(N integer), bits))
}

/// Performs rotate right.
#[inline]
#[track_caller]
pub(crate) const fn rotr<T: Uint, const N: usize>(integer: T, bits: u32) -> T {
  cast!(T api::rotr(cast!(N integer), bits))
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

/// Counts the number of bits set in an integer type `T`.
#[inline]
#[track_caller]
pub(crate) const fn ctpop<T: Uint, const N: usize>(integer: T) -> u32 {
  api::ctpop(cast!(N integer))
}

/// Counts the number of leading unset bits (zeroes) in an integer type `T`.
#[inline]
#[track_caller]
pub(crate) const fn ctlz<T: Uint, const N: usize>(integer: T) -> u32 {
  api::ctlz(cast!(N integer))
}

/// Counts the number of trailing unset bits (zeroes) in an integer type `T`.
#[inline]
#[track_caller]
pub(crate) const fn cttz<T: Uint, const N: usize>(integer: T) -> u32 {
  api::cttz(cast!(N integer))
}

/// Counts the number of leading unset bits (zeroes) in an integer type `T`.
///
/// # Safety
///
/// This results in undefined behavior when given an `integer` with value `0`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn ctlz_nonzero<T: Uint, const N: usize>(integer: T) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { api::ctlz_nonzero(cast!(N integer)) }
}

/// Counts the number of trailing unset bits (zeroes) in an integer type `T`.
///
/// # Safety
///
/// This results in undefined behavior when given an `integer` with value `0`.
#[allow(dead_code, reason = "Not currently used")]
#[inline]
#[track_caller]
pub(crate) const unsafe fn cttz_nonzero<T: Uint, const N: usize>(integer: T) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { api::cttz_nonzero(cast!(N integer)) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

/// Performs unsigned integer addition, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub(crate) const fn overflowing_uadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  cast!((T, bool) api::overflowing_uadd(cast!(N lhs), cast!(N rhs)))
}

/// Performs unsigned integer subtraction, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub(crate) const fn overflowing_usub<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  cast!((T, bool) api::overflowing_usub(cast!(N lhs), cast!(N rhs)))
}

/// Performs unsigned integer multiplication, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub(crate) const fn overflowing_umul<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  cast!((T, bool) api::overflowing_umul(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed integer addition, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub(crate) const fn overflowing_sadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  cast!((T, bool) api::overflowing_sadd(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed integer subtraction, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub(crate) const fn overflowing_ssub<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  cast!((T, bool) api::overflowing_ssub(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed integer multiplication, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub(crate) const fn overflowing_smul<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  cast!((T, bool) api::overflowing_smul(cast!(N lhs), cast!(N rhs)))
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

/// Performs unsigned saturating integer addition.
#[inline]
#[track_caller]
pub(crate) const fn saturating_uadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::saturating_uadd(cast!(N lhs), cast!(N rhs)))
}

/// Performs unsigned saturating integer subtraction.
#[inline]
#[track_caller]
pub(crate) const fn saturating_usub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::saturating_usub(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed saturating integer addition.
#[inline]
#[track_caller]
pub(crate) const fn saturating_sadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::saturating_sadd(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed saturating integer subtraction.
#[inline]
#[track_caller]
pub(crate) const fn saturating_ssub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::saturating_ssub(cast!(N lhs), cast!(N rhs)))
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

/// Performs unsigned unchecked integer addition.
///
/// # Safety
///
/// This results in undefined behavior when `lhs + rhs > T::MAX`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_uadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_uadd(cast!(N lhs), cast!(N rhs)))
}

/// Performs unsigned unchecked integer subtraction.
///
/// # Safety
///
/// This results in undefined behavior when `lhs - rhs < 0`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_usub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_usub(cast!(N lhs), cast!(N rhs)))
}

/// Performs unsigned unchecked integer multiplication.
///
/// # Safety
///
/// This results in undefined behavior when `lhs * rhs > T::MAX`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_umul<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_umul(cast!(N lhs), cast!(N rhs)))
}

/// Performs unsigned unchecked integer division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_udiv<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_udiv(cast!(N lhs), cast!(N rhs)))
}

/// Returns the remainder of an unsigned unchecked division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_urem<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_urem(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed unchecked integer addition.
///
/// # Safety
///
/// This results in undefined behavior when `lhs + rhs > T::MAX` or `lhs + rhs < T::MIN`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_sadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_sadd(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed unchecked integer subtraction.
///
/// # Safety
///
/// This results in undefined behavior when `lhs - rhs > T::MAX` or `lhs - rhs < T::MIN`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_ssub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_ssub(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed unchecked integer multiplication.
///
/// # Safety
///
/// This results in undefined behavior when `lhs * rhs > T::MAX` or `lhs * rhs < T::MIN`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_smul<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_smul(cast!(N lhs), cast!(N rhs)))
}

/// Performs signed unchecked integer division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_sdiv<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_sdiv(cast!(N lhs), cast!(N rhs)))
}

/// Returns the remainder of a signed unchecked division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_srem<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::unchecked_srem(cast!(N lhs), cast!(N rhs)))
}

/// Performs an unchecked left shift.
///
/// # Safety
///
/// This results in undefined behavior when `rhs >= N`, where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_shl<T: Uint, const N: usize>(integer: T, bits: u32) -> T {
  cast!(T api::unchecked_shl(cast!(N integer), bits))
}

/// Performs an unchecked logical right shift.
///
/// # Safety
///
/// This results in undefined behavior when `rhs >= N`, where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_lshr<T: Uint, const N: usize>(integer: T, bits: u32) -> T {
  cast!(T api::unchecked_lshr(cast!(N integer), bits))
}

/// Performs an unchecked arithmetic right shift.
///
/// # Safety
///
/// This results in undefined behavior when `rhs >= N`, where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub(crate) const unsafe fn unchecked_ashr<T: Uint, const N: usize>(integer: T, bits: u32) -> T {
  cast!(T api::unchecked_ashr(cast!(N integer), bits))
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

/// Performs integer addition, returning the result as mod 2<sup>N</sup>,
/// where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub(crate) const fn wrapping_add<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::wrapping_add(cast!(N lhs), cast!(N rhs)))
}

/// Performs integer subtraction, returning the result as mod 2<sup>N</sup>,
/// where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub(crate) const fn wrapping_sub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::wrapping_sub(cast!(N lhs), cast!(N rhs)))
}

/// Performs integer multiplication, returning the result as mod 2<sup>N</sup>,
/// where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub(crate) const fn wrapping_mul<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  cast!(T api::wrapping_mul(cast!(N lhs), cast!(N rhs)))
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

/// Performs full-width multiplication and addition with a carry: `lhs * rhs + add + carry`.
#[cfg(feature = "bigint_helper_methods")]
#[inline]
#[track_caller]
pub(crate) const fn carrying_mul_add<T: Uint, U: Uint, const N: usize>(
  _lhs: T,
  _rhs: T,
  _carry: T,
  _add: T,
) -> (U, T) {
  ::core::panic!("carrying_mul_add")
}
