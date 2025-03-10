use ::core::cmp::Ordering;

use crate::macros::maybe_intrinsic;
use crate::traits::Consts;
use crate::utils::is_negative;
use crate::utils::Index;

macro_rules! limbs {
  ($size:ident, $expr:expr) => {
    if $size % 8 == 0 {
      type Limbs<const N: usize> = $crate::limbs::Limbs64<N>;
      $expr
    } else if $size % 4 == 0 {
      type Limbs<const N: usize> = $crate::limbs::Limbs32<N>;
      $expr
    } else if $size % 2 == 0 {
      type Limbs<const N: usize> = $crate::limbs::Limbs16<N>;
      $expr
    } else {
      type Limbs<const N: usize> = $crate::limbs::Limbs8<N>;
      $expr
    }
  };
}

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
    let lhs: u8 = lhs[Index(index).msb::<N>()];
    let rhs: u8 = rhs[Index(index).msb::<N>()];

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
pub(crate) const fn scmp<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> Ordering {
  match (is_negative(lhs), is_negative(rhs)) {
    (true, false) => Ordering::Less,
    (false, true) => Ordering::Greater,
    (true, true) | (false, false) => ucmp(lhs, rhs),
  }
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
pub(crate) const fn rotl<const N: usize>(integer: [u8; N], bits: u32) -> [u8; N] {
  let bit_count: u32 = <[u8; N] as Consts>::BITS;
  let lhs_shift: u32 = bits % bit_count;
  let rhs_shift: u32 = (bit_count - lhs_shift) % bit_count;

  // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
  let lhs_value: [u8; N] = unsafe { unchecked_shl(integer, lhs_shift) };

  // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
  let rhs_value: [u8; N] = unsafe { unchecked_lshr(integer, rhs_shift) };

  bor(lhs_value, rhs_value)
}

#[inline]
pub(crate) const fn rotr<const N: usize>(integer: [u8; N], bits: u32) -> [u8; N] {
  let bit_count: u32 = <[u8; N] as Consts>::BITS;
  let lhs_shift: u32 = bits % bit_count;
  let rhs_shift: u32 = (bit_count - lhs_shift) % bit_count;

  // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
  let lhs_value: [u8; N] = unsafe { unchecked_lshr(integer, lhs_shift) };

  // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
  let rhs_value: [u8; N] = unsafe { unchecked_shl(integer, rhs_shift) };

  bor(lhs_value, rhs_value)
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
    match integer[Index(index).msb::<N>()] {
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
    match integer[Index(index).lsb::<N>()] {
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
  limbs!(N, Limbs::overflowing_uadd(lhs, rhs))
}

#[inline]
pub(crate) const fn overflowing_usub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  limbs!(N, Limbs::overflowing_usub(lhs, rhs))
}

#[inline]
pub(crate) const fn overflowing_umul<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  limbs!(N, Limbs::overflowing_umul(lhs, rhs))
}

#[inline]
pub(crate) const fn overflowing_sadd<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  limbs!(N, Limbs::overflowing_sadd(lhs, rhs))
}

#[inline]
pub(crate) const fn overflowing_ssub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  limbs!(N, Limbs::overflowing_ssub(lhs, rhs))
}

#[inline]
pub(crate) const fn overflowing_smul<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
  limbs!(N, Limbs::overflowing_smul(lhs, rhs))
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
pub(crate) const fn saturating_sadd<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  match overflowing_sadd(lhs, rhs) {
    (_value, true) if is_negative(lhs) => Consts::SMIN,
    (_value, true) => Consts::SMAX,
    (value, false) => value,
  }
}

#[inline]
pub(crate) const fn saturating_ssub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  match overflowing_ssub(lhs, rhs) {
    (_value, true) if is_negative(lhs) => Consts::SMIN,
    (_value, true) => Consts::SMAX,
    (value, false) => value,
  }
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
pub(crate) const unsafe fn unchecked_udiv<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::udivrem(lhs, rhs).0)
}

#[inline]
pub(crate) const unsafe fn unchecked_urem<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::udivrem(lhs, rhs).1)
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
pub(crate) const unsafe fn unchecked_sdiv<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::sdivrem(lhs, rhs).0)
}

#[inline]
pub(crate) const unsafe fn unchecked_srem<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::sdivrem(lhs, rhs).1)
}

#[inline]
pub(crate) const unsafe fn unchecked_shl<const N: usize>(integer: [u8; N], bits: u32) -> [u8; N] {
  unsafe {
    ::core::hint::assert_unchecked(bits < <[u8; N] as Consts>::BITS);
  }

  let large: usize = (bits as usize) >> u8::BITS.trailing_zeros();
  let small: u32 = bits & (u8::BITS - 1);
  let extra: u32 = u8::BITS - small;

  let mut value: [u8; N] = Consts::UMIN;

  if bits == 0 {
    integer
  } else if small == 0 {
    let mut index: usize = large;

    while index < N {
      value[Index(index).lsb::<N>()] = integer[Index(index - large).lsb::<N>()];
      index += 1;
    }

    value
  } else {
    let mut carry: u8 = 0;
    let mut index: usize = large;

    while index < N {
      let shift: u8 = integer[Index(index - large).lsb::<N>()];

      value[Index(index).lsb::<N>()] = (shift << small) | carry;
      carry = shift >> extra;
      index += 1;
    }

    value
  }
}

#[inline]
pub(crate) const unsafe fn unchecked_lshr<const N: usize>(integer: [u8; N], bits: u32) -> [u8; N] {
  unsafe {
    ::core::hint::assert_unchecked(bits < <[u8; N] as Consts>::BITS);
  }

  let large: usize = (bits as usize) >> u8::BITS.trailing_zeros();
  let small: u32 = bits & (u8::BITS - 1);
  let extra: u32 = u8::BITS - small;

  let mut value: [u8; N] = Consts::UMIN;

  if bits == 0 {
    integer
  } else if small == 0 {
    let mut index: usize = N;

    while index > large {
      index -= 1;
      value[Index(index - large).lsb::<N>()] = integer[Index(index).lsb::<N>()];
    }

    value
  } else {
    let mut carry: u8 = 0;
    let mut index: usize = N;

    while index > large {
      index -= 1;

      let shift: u8 = integer[Index(index).lsb::<N>()];

      value[Index(index - large).lsb::<N>()] = (shift >> small) | carry;
      carry = shift << extra;
    }

    value
  }
}

#[inline]
pub(crate) const unsafe fn unchecked_ashr<const N: usize>(integer: [u8; N], bits: u32) -> [u8; N] {
  unsafe {
    ::core::hint::assert_unchecked(bits < <[u8; N] as Consts>::BITS);
  }

  let large: usize = (bits as usize) >> u8::BITS.trailing_zeros();
  let small: u32 = bits & (u8::BITS - 1);
  let extra: u32 = u8::BITS - small;

  let mut value: [u8; N] = if is_negative(integer) {
    Consts::UMAX
  } else {
    Consts::UMIN
  };

  if bits == 0 {
    integer
  } else if small == 0 {
    let mut index: usize = N;

    while index > large {
      index -= 1;
      value[Index(index - large).lsb::<N>()] = integer[Index(index).lsb::<N>()];
    }

    value
  } else {
    let mut carry: u8 = 0;
    let mut index: usize = N;

    while index > large {
      index -= 1;

      let shift: u8 = integer[Index(index).lsb::<N>()];

      value[Index(index - large).lsb::<N>()] = if index == N - 1 {
        (shift as i8 >> small) as u8 | carry
      } else {
        (shift >> small) | carry
      };

      carry = shift << extra;
    }

    value
  }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

#[inline]
pub(crate) const fn wrapping_add<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::wrapping_add(lhs, rhs))
}

#[inline]
pub(crate) const fn wrapping_sub<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::wrapping_sub(lhs, rhs))
}

#[inline]
pub(crate) const fn wrapping_mul<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
  limbs!(N, Limbs::wrapping_mul(lhs, rhs))
}
