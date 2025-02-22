use ::core::cmp::Ordering;

use crate::llapi::macros::maybe_intrinsic;
use crate::llapi::macros::read_lsb;
use crate::llapi::macros::read_msb;
use crate::llapi::traits::Consts;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn band<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  let mut value: [u8; N] = Consts::UMIN;
  let mut index: usize = 0;

  while index < N {
    value[index] = lhs[index] & rhs[index];
    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn bor<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  let mut value: [u8; N] = Consts::UMIN;
  let mut index: usize = 0;

  while index < N {
    value[index] = lhs[index] | rhs[index];
    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn bxor<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  let mut value: [u8; N] = Consts::UMIN;
  let mut index: usize = 0;

  while index < N {
    value[index] = lhs[index] ^ rhs[index];
    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn bnot<const N: usize>(integer: [u8; N]) -> [u8; N] {
  bxor(integer, Consts::UMAX)
}

#[cfg(feature = "disjoint_bitor")]
#[inline]
pub(crate) const unsafe fn disjoint_bor<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  unsafe {
    ::core::hint::assert_unchecked(eq(band(lhs, rhs), Consts::UMIN));
  }

  bor(lhs, rhs)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn eq<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> bool {
  // SAFETY: byte arrays do not have padding or uninitialized memory.
  maybe_intrinsic! {
    @enabled => unsafe {
      ::core::intrinsics::raw_eq(&lhs, &rhs)
    }
    @default => {
      let mut value: bool = true;
      let mut index: usize = 0;

      while index < N {
        // This seems to optimize better than `if lhs != rhs`
        value &= lhs[index] == rhs[index];
        index += 1;
      }

      value
    }
  }
}

#[inline]
pub(crate) const fn ucmp<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> Ordering {
  let mut index: usize = 0;

  while index < N {
    let lhs: u8 = read_msb!(N, lhs, index);
    let rhs: u8 = read_msb!(N, rhs, index);

    if lhs < rhs {
      return Ordering::Less;
    } else if lhs > rhs {
      return Ordering::Greater;
    }

    index += 1;
  }

  Ordering::Equal
}

#[inline]
pub(crate) const fn scmp<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> Ordering {
  ::core::panic!("scmp")
}

// -----------------------------------------------------------------------------
// Bit Conversion Operation
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn swap1<const N: usize>(integer: [u8; N]) -> [u8; N] {
  let mut value: [u8; N] = Consts::UMIN;
  let mut index: usize = 0;

  while index < N {
    value[N - 1 - index] = integer[index].reverse_bits();
    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn swap8<const N: usize>(integer: [u8; N]) -> [u8; N] {
  let mut value: [u8; N] = Consts::UMIN;
  let mut index: usize = 0;

  while index < N {
    value[N - 1 - index] = integer[index];
    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn rotl<const N: usize>(_integer: [u8; N], _bits: u32) -> [u8; N] {
  ::core::panic!("rotl")
}

#[inline]
pub(crate) const fn rotr<const N: usize>(_integer: [u8; N], _bits: u32) -> [u8; N] {
  ::core::panic!("rotr")
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn ctpop<const N: usize>(integer: [u8; N]) -> u32 {
  let mut value: u32 = 0;
  let mut index: usize = 0;

  while index < N {
    value += integer[index].count_ones();
    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn ctlz<const N: usize>(integer: [u8; N]) -> u32 {
  let mut value: u32 = 0;
  let mut index: usize = 0;

  while index < N {
    match read_msb!(N, integer, index) {
      0 => value += u8::BITS,
      v => return value + v.leading_zeros(),
    }

    index += 1;
  }

  value
}

#[inline]
pub(crate) const fn cttz<const N: usize>(integer: [u8; N]) -> u32 {
  let mut value: u32 = 0;
  let mut index: usize = 0;

  while index < N {
    match read_lsb!(N, integer, index) {
      0 => value += u8::BITS,
      v => return value + v.trailing_zeros(),
    }

    index += 1;
  }

  value
}

#[inline]
pub(crate) const unsafe fn ctlz_nonzero<const N: usize>(integer: [u8; N]) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe {
    ::core::hint::assert_unchecked(!eq(integer, Consts::UMIN));
  }

  ctlz(integer)
}

#[inline]
pub(crate) const unsafe fn cttz_nonzero<const N: usize>(integer: [u8; N]) -> u32 {
  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe {
    ::core::hint::assert_unchecked(!eq(integer, Consts::UMIN));
  }

  cttz(integer)
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn overflowing_uadd<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  let out: [u8; N] = wrapping_add(lhs, rhs);
  let cmp: bool = ucmp(out, lhs).is_lt();

  (out, cmp)
}

#[inline]
pub(crate) const fn overflowing_usub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  let out: [u8; N] = wrapping_sub(lhs, rhs);
  let cmp: bool = ucmp(lhs, rhs).is_lt();

  (out, cmp)
}

#[inline]
pub(crate) const fn overflowing_umul<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> ([u8; N], bool) {
  ::core::panic!("overflowing_umul")
}

#[inline]
pub(crate) const fn overflowing_sadd<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> ([u8; N], bool) {
  ::core::panic!("overflowing_sadd")
}

#[inline]
pub(crate) const fn overflowing_ssub<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> ([u8; N], bool) {
  ::core::panic!("overflowing_ssub")
}

#[inline]
pub(crate) const fn overflowing_smul<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> ([u8; N], bool) {
  ::core::panic!("overflowing_smul")
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn saturating_uadd<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  match overflowing_uadd(lhs, rhs) {
    (_value, true) => Consts::UMAX,
    (value, false) => value,
  }
}

#[inline]
pub(crate) const fn saturating_usub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  match overflowing_usub(lhs, rhs) {
    (_value, true) => Consts::UMIN,
    (value, false) => value,
  }
}

#[inline]
pub(crate) const fn saturating_sadd<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("saturating_sadd")
}

#[inline]
pub(crate) const fn saturating_ssub<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("saturating_ssub")
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const unsafe fn unchecked_uadd<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  wrapping_add(lhs, rhs)
}

#[inline]
pub(crate) const unsafe fn unchecked_usub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  wrapping_sub(lhs, rhs)
}

#[inline]
pub(crate) const unsafe fn unchecked_umul<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  wrapping_mul(lhs, rhs)
}

#[inline]
pub(crate) const unsafe fn unchecked_udiv<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("unchecked_udiv")
}

#[inline]
pub(crate) const unsafe fn unchecked_urem<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("unchecked_urem")
}

#[inline]
pub(crate) const unsafe fn unchecked_sadd<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  wrapping_add(lhs, rhs)
}

#[inline]
pub(crate) const unsafe fn unchecked_ssub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  wrapping_sub(lhs, rhs)
}

#[inline]
pub(crate) const unsafe fn unchecked_smul<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  wrapping_mul(lhs, rhs)
}

#[inline]
pub(crate) const unsafe fn unchecked_sdiv<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("unchecked_sdiv")
}

#[inline]
pub(crate) const unsafe fn unchecked_srem<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("unchecked_srem")
}

#[inline]
pub(crate) const unsafe fn unchecked_shl<const N: usize>(_integer: [u8; N], _bits: u32) -> [u8; N] {
  ::core::panic!("unchecked_shl")
}

#[inline]
pub(crate) const unsafe fn unchecked_lshr<const N: usize>(_integer: [u8; N], _bits: u32) -> [u8; N] {
  ::core::panic!("unchecked_lshr")
}

#[inline]
pub(crate) const unsafe fn unchecked_ashr<const N: usize>(_integer: [u8; N], _bits: u32) -> [u8; N] {
  ::core::panic!("unchecked_ashr")
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn wrapping_add<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("wrapping_add")
}

#[inline]
pub(crate) const fn wrapping_sub<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("wrapping_sub")
}

#[inline]
pub(crate) const fn wrapping_mul<const N: usize>(_lhs: [u8; N], _rhs: [u8; N]) -> [u8; N] {
  ::core::panic!("wrapping_mul")
}
