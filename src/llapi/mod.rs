//! Exint Low-Level API
//!
//! These are the specialized implementations of all fundamental algorithms used
//! throughout the library.
//!
//! Most of the functions exposed through this API use delegated implementations
//! through the `specialize!` macro. These functions typically expect a type
//! `T: Uint` that upholds all invariants of the required unsafe operations.
//!
//! See [`Uint`] for more information on these invariants.
//!
//! Notable unsafe code includes the `input!` and `output!` macros which
//! contain "unchecked" transmutes to cast `T` from/into `<some specialized type>`.
//!
//! The required invariants for these conversions can be found in the module docs
//! of the relevant specialized implementations in llapi/specialize/*.rs

#![expect(unused_unsafe, reason = "macro-generated code")]

use ::core::cmp::Ordering;

mod intrinsics;
mod specialize;
mod utils;

pub use self::utils::Uint;

macro_rules! input {
  (@pure $name:ident) => {
    $name
  };
  ($name:ident) => {
    unsafe { $crate::llapi::utils::transmute::<T, _>($name) }
  };
}

#[allow(unused_macro_rules, reason = "feature-dependant")]
macro_rules! output {
  (@pure $value:expr) => {
    $value
  };
  (@cast($var:ident) $value:expr) => {
    unsafe { $crate::llapi::utils::transmute::<_, $var>($value) }
  };
  (@cast($var:ident, bool) $value:expr) => {
    (output!(@cast($var) $value.0), $value.1)
  };
  (@cast($a:ident, $b:ident) $value:expr) => {
    (output!(@cast($a) $value.0), output!(@cast($b) $value.1))
  };
}

macro_rules! branch {
  (
    $transform:ident$(::<$($generic:tt),+>)?::$method:ident($($(@$arg_conv:ident)? $arg_name:ident),+)
    ->
    @$ret_conv:ident$(($($ret_type:ident),+))?
  ) => {{
    $(
      let $arg_name = input!($(@$arg_conv)? $arg_name);
    )+

    let result = $crate::llapi::specialize::$transform$(::<$($generic),+>)?::$method(
      $($arg_name),+
    );

    output!(@$ret_conv$(($($ret_type),+))? result)
  }};
}

macro_rules! specialize {
  (Int::$method:ident($($args:tt)+)) => {
    specialize!(Int::$method($($args)+) -> @cast(T))
  };
  (Int::$method:ident($($args:tt)+) -> @$ret_conv:ident$(($($ret_type:ident),+))?) => {
    match N {
      // -----------------------------------------------------------------------
      // Standard Sizes
      // -----------------------------------------------------------------------
      1 => branch!(I8::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      2 => branch!(I16::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      4 => branch!(I32::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      8 => branch!(I64::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      16 => branch!(I128::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      // -----------------------------------------------------------------------
      // Extended Sizes
      // -----------------------------------------------------------------------
      3 => branch!(I24::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      5 => branch!(I40::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      6 => branch!(I48::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      7 => branch!(I56::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      9 => branch!(I72::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      10 => branch!(I80::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      11 => branch!(I88::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      12 => branch!(I96::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      13 => branch!(I104::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      14 => branch!(I112::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      15 => branch!(I120::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      // -----------------------------------------------------------------------
      // SIMD-Supported Sizes
      // -----------------------------------------------------------------------
      #[cfg(feature = "portable_simd")]
      32 => branch!(I64xN::<32, 4>::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      #[cfg(feature = "portable_simd")]
      48 => branch!(I64xN::<48, 6>::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      #[cfg(feature = "portable_simd")]
      64 => branch!(I64xN::<64, 8>::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      #[cfg(feature = "portable_simd")]
      128 => branch!(I64xN::<128, 16>::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
      // -----------------------------------------------------------------------
      // Fallback
      // -----------------------------------------------------------------------
      _ => branch!(Int::<N>::$method($($args)+) -> @$ret_conv$(($($ret_type),+))?),
    }
  };
}

// -----------------------------------------------------------------------------
// Compiler Hints
// -----------------------------------------------------------------------------

#[cfg(feature = "core_intrinsics")]
pub use ::core::intrinsics::unlikely;

/// A hint to the compiler that the given `condition` is likely to be `false`.
#[cfg(not(feature = "core_intrinsics"))]
#[inline]
#[track_caller]
pub const fn unlikely(condition: bool) -> bool {
  #[cold]
  const fn cold() {}

  if condition {
    cold();
    true
  } else {
    false
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
pub const fn cast_bytes<const UINT: bool, const N: usize, const M: usize>(
  bytes: [u8; N],
) -> [u8; M] {
  let mut out: [u8; M] = [0; M];
  let mut src: *const u8 = bytes.as_ptr();
  let mut dst: *mut u8 = out.as_mut_ptr();

  if N < M {
    // On BE machines, we need to shift the dst pointer to a higher memory
    // address and leave the less-significant bytes untouched.
    if ::core::cfg!(target_endian = "big") {
      // SAFETY: We are still within the bounds of valid memory for `dst`
      //         because `dst == [u8; M]` and `(M - N) + N == M`.
      dst = unsafe { dst.add(M - N) };
    }

    // SAFETY: We know `N < M` since we ended up in this branch, this means
    //         `src` and `dst` are both valid for reads and writes of `N` bytes.
    //         We also know both pointers are plain byte arrays with identical
    //         alignment. Finally, since `src` was not yet deallocated, we can
    //         be sure the values do not overlap.
    unsafe { ::core::ptr::copy_nonoverlapping(src, dst, N) }

    if !UINT {
      // TODO: This will fail for really large extensions
      //
      // NOTE: Optimizing this pattern seems heavily context-dependant and
      //       doesn't really optimize well for conversion to signed integers
      //       when `shift >= 72`
      let shift: u32 = ((M - N) * 8) as u32;

      // SAFETY: We know that `shift` is less than the bid width of `out` since
      //         `N < M` and `(M - N) * 8 < M * 8`.
      out = unsafe { unchecked_shl::<_, M>(out, shift) };
      out = unsafe { unchecked_ashr::<_, M>(out, shift) };
    }
  } else {
    // On BE machines, we need to shift the src pointer to a higher memory
    // address and copy the less-significant bytes.
    if ::core::cfg!(target_endian = "big") {
      // SAFETY: We are still within the bounds of valid memory for `src`
      //         because `src == [u8; N]` and `(N - M) + M == N`.
      src = unsafe { src.add(N - M) };
    }

    // SAFETY: We know `N >= M` since we ended up in this branch, this means
    //         `src` and `dst` are both valid for reads and writes of `M` bytes.
    //         We also know both pointers are plain byte arrays with identical
    //         alignment. Finally, since `src` was not yet deallocated, we can
    //         be sure the values do not overlap.
    unsafe { ::core::ptr::copy_nonoverlapping(src, dst, M) }
  }

  out
}

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

/// Performs bitwise logical `AND`.
#[inline]
#[track_caller]
pub const fn band<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::band(lhs, rhs))
}

/// Performs bitwise logical inclusive `OR`.
#[inline]
#[track_caller]
pub const fn bor<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::bor(lhs, rhs))
}

/// Performs bitwise logical exclusive `OR`.
#[inline]
#[track_caller]
pub const fn bxor<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::bxor(lhs, rhs))
}

/// Performs bitwise logical negation.
#[inline]
#[track_caller]
pub const fn bnot<T: Uint, const N: usize>(int: T) -> T {
  specialize!(Int::bnot(int))
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

/// Tests if two integer values are equal.
#[inline]
#[track_caller]
pub const fn eq<T: Uint, const N: usize>(lhs: T, rhs: T) -> bool {
  specialize!(Int::eq(lhs, rhs) -> @pure)
}

/// Returns an ordering between two unsigned integer values.
#[inline]
#[track_caller]
pub const fn ucmp<T: Uint, const N: usize>(lhs: T, rhs: T) -> Ordering {
  specialize!(Int::ucmp(lhs, rhs) -> @pure)
}

/// Returns an ordering between two signed integer values.
#[inline]
#[track_caller]
pub const fn scmp<T: Uint, const N: usize>(lhs: T, rhs: T) -> Ordering {
  specialize!(Int::scmp(lhs, rhs) -> @pure)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

/// Reverses the bits in an integer type `T`.
#[inline]
#[track_caller]
pub const fn swap1<T: Uint, const N: usize>(int: T) -> T {
  specialize!(Int::swap1(int))
}

/// Reverses the bytes in an integer type `T`.
#[inline]
#[track_caller]
pub const fn swap8<T: Uint, const N: usize>(int: T) -> T {
  specialize!(Int::swap8(int))
}

/// Performs rotate left.
#[inline]
#[track_caller]
pub const fn rotl<T: Uint, const N: usize>(int: T, bits: u32) -> T {
  specialize!(Int::rotl(int, @pure bits))
}

/// Performs rotate right.
#[inline]
#[track_caller]
pub const fn rotr<T: Uint, const N: usize>(int: T, bits: u32) -> T {
  specialize!(Int::rotr(int, @pure bits))
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

/// Counts the number of bits set in an integer type `T`.
#[inline]
#[track_caller]
pub const fn ctpop<T: Uint, const N: usize>(int: T) -> u32 {
  specialize!(Int::ctpop(int) -> @pure)
}

/// Counts the number of leading unset bits (zeroes) in an integer type `T`.
#[inline]
#[track_caller]
pub const fn ctlz<T: Uint, const N: usize>(int: T) -> u32 {
  specialize!(Int::ctlz(int) -> @pure)
}

/// Counts the number of trailing unset bits (zeroes) in an integer type `T`.
#[inline]
#[track_caller]
pub const fn cttz<T: Uint, const N: usize>(int: T) -> u32 {
  specialize!(Int::cttz(int) -> @pure)
}

/// Counts the number of leading unset bits (zeroes) in an integer type `T`.
///
/// # Safety
///
/// This results in undefined behavior when given an `integer` with value `0`.
#[inline]
#[track_caller]
pub const unsafe fn ctlz_nonzero<T: Uint, const N: usize>(int: T) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::ctlz_nonzero(int) -> @pure) }
}

/// Counts the number of trailing unset bits (zeroes) in an integer type `T`.
///
/// # Safety
///
/// This results in undefined behavior when given an `integer` with value `0`.
#[inline]
#[track_caller]
pub const unsafe fn cttz_nonzero<T: Uint, const N: usize>(int: T) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::cttz_nonzero(int) -> @pure) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

/// Performs unsigned integer addition, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub const fn overflowing_uadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  specialize!(Int::overflowing_uadd(lhs, rhs) -> @cast(T, bool))
}

/// Performs unsigned integer subtraction, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub const fn overflowing_usub<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  specialize!(Int::overflowing_usub(lhs, rhs) -> @cast(T, bool))
}

/// Performs unsigned integer multiplication, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub const fn overflowing_umul<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  specialize!(Int::overflowing_umul(lhs, rhs) -> @cast(T, bool))
}

/// Performs signed integer addition, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub const fn overflowing_sadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  specialize!(Int::overflowing_sadd(lhs, rhs) -> @cast(T, bool))
}

/// Performs signed integer subtraction, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub const fn overflowing_ssub<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  specialize!(Int::overflowing_ssub(lhs, rhs) -> @cast(T, bool))
}

/// Performs signed integer multiplication, returning an indicator whether overflow occured.
#[inline]
#[track_caller]
pub const fn overflowing_smul<T: Uint, const N: usize>(lhs: T, rhs: T) -> (T, bool) {
  specialize!(Int::overflowing_smul(lhs, rhs) -> @cast(T, bool))
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

/// Performs unsigned saturating integer addition.
#[inline]
#[track_caller]
pub const fn saturating_uadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::saturating_uadd(lhs, rhs))
}

/// Performs unsigned saturating integer subtraction.
#[inline]
#[track_caller]
pub const fn saturating_usub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::saturating_usub(lhs, rhs))
}

/// Performs signed saturating integer addition.
#[inline]
#[track_caller]
pub const fn saturating_sadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::saturating_sadd(lhs, rhs))
}

/// Performs signed saturating integer subtraction.
#[inline]
#[track_caller]
pub const fn saturating_ssub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::saturating_ssub(lhs, rhs))
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
pub const unsafe fn unchecked_uadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_uadd(lhs, rhs)) }
}

/// Performs unsigned unchecked integer subtraction.
///
/// # Safety
///
/// This results in undefined behavior when `lhs - rhs < 0`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_usub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_usub(lhs, rhs)) }
}

/// Performs unsigned unchecked integer multiplication.
///
/// # Safety
///
/// This results in undefined behavior when `lhs * rhs > T::MAX`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_umul<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_umul(lhs, rhs)) }
}

/// Performs unsigned unchecked integer division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_udiv<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_udiv(lhs, rhs)) }
}

/// Performs an exact division.
///
/// # Safety
///
/// This results in undefined behavior when `lhs % rhs != 0` or `rhs == 0`.
#[cfg(feature = "exact_div")]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_udiv_exact<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_udiv_exact(lhs, rhs)) }
}

/// Returns the remainder of an unsigned unchecked division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_urem<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_urem(lhs, rhs)) }
}

/// Performs signed unchecked integer addition.
///
/// # Safety
///
/// This results in undefined behavior when `lhs + rhs > T::MAX` or `lhs + rhs < T::MIN`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_sadd<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_sadd(lhs, rhs)) }
}

/// Performs signed unchecked integer subtraction.
///
/// # Safety
///
/// This results in undefined behavior when `lhs - rhs > T::MAX` or `lhs - rhs < T::MIN`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_ssub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_ssub(lhs, rhs)) }
}

/// Performs signed unchecked integer multiplication.
///
/// # Safety
///
/// This results in undefined behavior when `lhs * rhs > T::MAX` or `lhs * rhs < T::MIN`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_smul<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_smul(lhs, rhs)) }
}

/// Performs signed unchecked integer division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_sdiv<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_sdiv(lhs, rhs)) }
}

/// Performs an exact division.
///
/// # Safety
///
/// This results in undefined behavior when `lhs % rhs != 0`, `rhs == 0`, or `lhs == T::MIN && rhs == -1`.
#[cfg(feature = "exact_div")]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_sdiv_exact<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_sdiv_exact(lhs, rhs)) }
}

/// Returns the remainder of a signed unchecked division.
///
/// # Safety
///
/// This results in undefined behavior when `rhs == 0` or `lhs == T::MIN && rhs == -1`.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_srem<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_srem(lhs, rhs)) }
}

/// Performs an unchecked left shift.
///
/// # Safety
///
/// This results in undefined behavior when `rhs >= N`, where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_shl<T: Uint, const N: usize>(int: T, bits: u32) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_shl(int, @pure bits)) }
}

/// Performs an unchecked logical right shift.
///
/// # Safety
///
/// This results in undefined behavior when `rhs >= N`, where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_lshr<T: Uint, const N: usize>(int: T, bits: u32) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_lshr(int, @pure bits)) }
}

/// Performs an unchecked arithmetic right shift.
///
/// # Safety
///
/// This results in undefined behavior when `rhs >= N`, where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub const unsafe fn unchecked_ashr<T: Uint, const N: usize>(int: T, bits: u32) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_ashr(int, @pure bits)) }
}

/// Performs an unchecked left funnel shift.
///
/// # Safety
///
/// This results in undefined behavior when `bits >= N`, where N is the width of `T` in bits.
#[cfg(feature = "funnel_shifts")]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_fshl<T: Uint, const N: usize>(lhs: T, rhs: T, bits: u32) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_fshl(lhs, rhs, @pure bits)) }
}

/// Performs an unchecked right funnel shift.
///
/// # Safety
///
/// This results in undefined behavior when `bits >= N`, where N is the width of `T` in bits.
#[cfg(feature = "funnel_shifts")]
#[inline]
#[track_caller]
pub const unsafe fn unchecked_fshr<T: Uint, const N: usize>(lhs: T, rhs: T, bits: u32) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::unchecked_fshr(lhs, rhs, @pure bits)) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

/// Performs integer addition, returning the result as mod 2<sup>N</sup>,
/// where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub const fn wrapping_add<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::wrapping_add(lhs, rhs))
}

/// Performs integer subtraction, returning the result as mod 2<sup>N</sup>,
/// where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub const fn wrapping_sub<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::wrapping_sub(lhs, rhs))
}

/// Performs integer multiplication, returning the result as mod 2<sup>N</sup>,
/// where N is the width of `T` in bits.
#[inline]
#[track_caller]
pub const fn wrapping_mul<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  specialize!(Int::wrapping_mul(lhs, rhs))
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

/// Performs bitwise logical inclusive `OR` with values having no common bits.
///
/// # Safety
///
/// This results in undefined behavior when `(lhs & rhs) != 0`.
#[cfg(feature = "disjoint_bitor")]
#[inline]
#[track_caller]
pub const unsafe fn disjoint_bor<T: Uint, const N: usize>(lhs: T, rhs: T) -> T {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { specialize!(Int::disjoint_bor(lhs, rhs)) }
}

/// Performs full-width multiplication and addition with a carry: `lhs * rhs + add + carry`.
#[cfg(feature = "bigint_helper_methods")]
#[inline]
#[track_caller]
pub const fn carrying_umul_uadd<T: Uint, const N: usize>(
  lhs: T,
  rhs: T,
  add: T,
  carry: T,
) -> (T, T) {
  specialize!(Int::carrying_umul_uadd(lhs, rhs, add, carry) -> @cast(T, T))
}

/// Performs full-width multiplication and addition with a carry: `lhs * rhs + add + carry`.
#[cfg(feature = "bigint_helper_methods")]
#[inline]
#[track_caller]
pub const fn carrying_smul_sadd<T: Uint, U: Uint, const N: usize>(
  lhs: T,
  rhs: T,
  add: T,
  carry: T,
) -> (U, T) {
  specialize!(Int::carrying_smul_sadd(lhs, rhs, add, carry) -> @cast(U, T))
}
