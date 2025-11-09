//! Integer Log Algorithms

use ::core::num::NonZero;

use crate::llapi::ctlz_nonzero;
use crate::types::uint;

/// Returns the logarithm of the number with respect to an arbitrary base, rounded down.
///
/// # Safety
///
/// This results in undefined behavior when the input is zero.
pub(crate) const unsafe fn ilog<const N: usize>(this: uint<N>, base: uint<N>) -> u32 {
  if this.const_lt(&base) {
    return 0;
  }

  let mut ilog: u32 = 1;
  let mut data: uint<N> = base;

  while data.const_le(&this.const_div(base)) {
    ilog += 1;
    data = data.const_mul(base);
  }

  ilog
}

/// Returns the base 2 logarithm of the number, rounded down.
///
/// # Safety
///
/// This results in undefined behavior when the input is zero.
#[must_use = must_use_doc!()]
#[inline]
pub(crate) const unsafe fn ilog2<const N: usize>(this: uint<N>) -> u32 {
  if const { N.is_power_of_two() } {
    // SAFETY: This is guaranteed to be safe by the caller.
    return uint::<N>::BITS - 1 - unsafe { ctlz_nonzero::<uint<N>, N>(this) };
  }

  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { ilog(this, uint::TWO) }
}

/// Returns the base 10 logarithm of the number, rounded down.
///
/// # Safety
///
/// This results in undefined behavior when the input is zero.
#[inline]
pub(crate) const unsafe fn ilog10<const N: usize>(this: uint<N>) -> u32 {
  match N {
    1 => {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { NonZero::new_unchecked(this.into_u8()) }.ilog10()
    }
    2 => {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { NonZero::new_unchecked(this.into_u16()) }.ilog10()
    }
    3..=4 => {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { NonZero::new_unchecked(this.into_u32()) }.ilog10()
    }
    5..=8 => {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { NonZero::new_unchecked(this.into_u64()) }.ilog10()
    }
    9..=16 => {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { NonZero::new_unchecked(this.into_u128()) }.ilog10()
    }
    _ => {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { ilog(this, uint::TEN) }
    }
  }
}
