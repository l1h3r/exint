#![allow(unused_imports, reason = "feature-dependant")]

use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::isqrt;
use crate::llapi;
use crate::panic;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "max_value")]
  #[deprecated(
    since = "TBD",
    note = "replaced by the `MAX` associated constant on this type"
  )]
  #[must_use]
  #[inline]
  pub const fn max_value() -> Self {
    Self::MAX
  }

  #[doc = include_doc!(uint, "min_value")]
  #[deprecated(
    since = "TBD",
    note = "replaced by the `MIN` associated constant on this type"
  )]
  #[must_use]
  #[inline]
  pub const fn min_value() -> Self {
    Self::MIN
  }

  #[doc = include_doc!(uint, "cast_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn cast_signed(self) -> int<N> {
    int::from_ne_bytes(self.to_ne_bytes())
  }

  #[doc = include_doc!(uint, "midpoint")]
  #[doc(alias = "average_floor")]
  #[doc(alias = "average")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn midpoint(self, rhs: Self) -> Self {
    self
      .const_bxor(rhs)
      .const_shr(1)
      .const_add(self.const_band(rhs))
  }

  #[doc = include_doc!(uint, "div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_euclid(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn rem_euclid(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "div_ceil")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_ceil(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if rem.const_gt(&Self::ZERO) {
      div.const_add(Self::ONE)
    } else {
      div
    }
  }

  #[doc = include_doc!(uint, "div_floor")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_floor(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "exact_div")]
  #[cfg(feature = "exact_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn exact_div(self, rhs: Self) -> Self {
    match self.checked_exact_div(rhs) {
      Some(value) => value,
      None if rhs.is_zero() => panic::div_zero(),
      None => panic::exact_div(),
    }
  }

  #[doc = include_doc!(uint, "shl_exact")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn shl_exact(self, rhs: u32) -> Option<Self> {
    if rhs <= self.leading_zeros() && rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shl(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(uint, "shr_exact")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn shr_exact(self, rhs: u32) -> Option<Self> {
    if rhs <= self.trailing_zeros() && rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shr(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(uint, "next_multiple_of")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn next_multiple_of(self, rhs: Self) -> Self {
    let rem: Self = self.const_rem(rhs);

    if rem.is_zero() {
      self
    } else {
      self.const_add(rhs.const_sub(rem))
    }
  }

  #[doc = include_doc!(uint, "pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn pow(self, mut exp: u32) -> Self {
    if exp == 0 {
      return Self::ONE;
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;

    #[cfg(feature = "core_intrinsics")]
    if ::core::intrinsics::is_val_statically_known(exp) {
      while exp > 1 {
        if (exp & 1) == 1 {
          acc = acc.const_mul(base);
        }

        exp /= 2;
        base = base.const_mul(base);
      }

      return acc.const_mul(base);
    }

    loop {
      if (exp & 1) == 1 {
        acc = acc.const_mul(base);

        if exp == 1 {
          return acc;
        }
      }

      exp /= 2;
      base = base.const_mul(base);
    }
  }

  #[doc = include_doc!(uint, "ilog")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog(self, base: Self) -> u32 {
    if base.const_lt(&Self::TWO) {
      panic::ilog_base();
    }

    match self.checked_ilog(base) {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(uint, "ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog2(self) -> u32 {
    match self.checked_ilog2() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(uint, "ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog10(self) -> u32 {
    match self.checked_ilog10() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(uint, "isqrt")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn isqrt(self) -> Self {
    let result: Self = isqrt::uint_sqrt(self);

    unsafe {
      let max: Self = const { isqrt::uint_sqrt(Self::MAX) };
      ::core::hint::assert_unchecked(result.const_le(&max));
    }

    result
  }

  #[doc = include_doc!(uint, "abs_diff")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs_diff(self, rhs: Self) -> Self {
    if N == 1 {
      // Trick LLVM into generating the psadbw instruction when SSE2
      // is available and this function is autovectorized for uint<1>'s.
      Self::from_u32(self.into_i32().wrapping_sub(rhs.into_i32()).unsigned_abs())
    } else if self.const_lt(&rhs) {
      rhs.const_sub(self)
    } else {
      self.const_sub(rhs)
    }
  }

  #[doc = include_doc!(uint, "next_power_of_two")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn next_power_of_two(self) -> Self {
    self.one_less_than_next_power_of_two().const_add(Self::ONE)
  }

  #[doc = include_doc!(uint, "is_power_of_two")]
  #[must_use]
  #[inline]
  pub const fn is_power_of_two(self) -> bool {
    self.count_ones() == 1
  }

  #[doc = include_doc!(uint, "is_multiple_of")]
  #[must_use]
  #[inline]
  pub const fn is_multiple_of(self, rhs: Self) -> bool {
    if rhs.is_zero() {
      self.is_zero()
    } else {
      self.const_rem(rhs).is_zero()
    }
  }

  #[doc = include_doc!(uint, "funnel_shl")]
  #[cfg(feature = "funnel_shifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn funnel_shl(self, rhs: Self, bits: u32) -> Self {
    if bits < Self::BITS {
      // SAFETY: We just ensured the shift is in-bounds.
      unsafe { llapi::unchecked_fshl::<Self, N>(self, rhs, bits) }
    } else {
      panic::fshl()
    }
  }

  #[doc = include_doc!(uint, "funnel_shr")]
  #[cfg(feature = "funnel_shifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn funnel_shr(self, rhs: Self, bits: u32) -> Self {
    if bits < Self::BITS {
      // SAFETY: We just ensured the shift is in-bounds.
      unsafe { llapi::unchecked_fshr::<Self, N>(self, rhs, bits) }
    } else {
      panic::fshr()
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_max_value, () => {
    assert_eq!(T::max_value(), T::MAX);
  });

  test!(@uint, test_min_value, () => {
    assert_eq!(T::min_value(), T::MIN);
  });

  test!(@uint, test_cast_signed, () => {
    assert_eq!(T::P_1.cast_signed(), U::from_i8(1));
    assert_eq!(T::P_2.cast_signed(), U::from_i8(2));
    assert_eq!(T::MIN.cast_signed(), U::from_i8(0));
    assert_eq!(T::MAX.cast_signed(), U::from_i8(-1));
  });

  test!(@uint, test_midpoint, () => {
    assert_eq!(T::MIN.midpoint(T::P_1), T::MIN);
    assert_eq!(T::MIN.midpoint(T::P_2), T::P_1);
    assert_eq!(T::MIN.midpoint(T::MIN), T::MIN);
    assert_eq!(T::MIN.midpoint(T::MAX), T::MAX / T::P_2);

    assert_eq!(T::MAX.midpoint(T::P_1), T::MAX / T::P_2 + T::P_1);
    assert_eq!(T::MAX.midpoint(T::P_2), T::MAX / T::P_2 + T::P_1);
    assert_eq!(T::MAX.midpoint(T::MIN), T::MAX / T::P_2);
    assert_eq!(T::MAX.midpoint(T::MAX), T::MAX);
  });

  test!(@uint, test_div_euclid, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_rem_euclid, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_div_ceil, () => {
    assert_eq!(T::P_1.div_ceil(T::P_2), T::P_1);
    assert_eq!(T::P_2.div_ceil(T::P_2), T::P_1);
    assert_eq!(T::MIN.div_ceil(T::P_2), T::MIN);
    assert_eq!(T::MAX.div_ceil(T::P_2), T::MAX / T::P_2 + T::P_1);

    assert_panic!(T::P_1.div_ceil(T::MIN), message = DIV_ZERO);
    assert_panic!(T::P_2.div_ceil(T::MIN), message = DIV_ZERO);
    assert_panic!(T::MIN.div_ceil(T::MIN), message = DIV_ZERO);
    assert_panic!(T::MAX.div_ceil(T::MIN), message = DIV_ZERO);
  });

  test!(@uint, test_div_floor, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_exact_div, () => {
    assert_panic!(T::P_1.exact_div(T::P_2), message = EXACT_DIV);
    assert_eq!(T::P_2.exact_div(T::P_2), T::P_1);
    assert_eq!(T::MIN.exact_div(T::P_2), T::MIN);
    assert_panic!(T::MAX.exact_div(T::P_2), message = EXACT_DIV);

    assert_panic!(T::P_1.exact_div(T::MIN), message = DIV_ZERO);
    assert_panic!(T::P_2.exact_div(T::MIN), message = DIV_ZERO);
    assert_panic!(T::MIN.exact_div(T::MIN), message = DIV_ZERO);
    assert_panic!(T::MAX.exact_div(T::MIN), message = DIV_ZERO);
  });

  test!(@uint, test_shl_exact, () => {
    assert_shift!(shl_exact, T, T::P_1, 1, 3, 5, S_1, S_2, S_3, S_4);
    assert_shift!(shl_exact, T, T::P_2, 1, 3, 5, None, S_2, S_3, S_4);
    assert_shift!(shl_exact, T, T::MIN, 0);
    assert_shift!(shl_exact, T, T::MAX, None);
  });

  test!(@uint, test_shr_exact, () => {
    assert_shift!(shr_exact, T, T::P_1, None);

    if T::SIZE == 1 {
      assert_shift!(shr_exact, T, T::P_2, 1, None, None, None, None, S_3, None);
    } else {
      assert_shift!(shr_exact, T, T::P_2, 1, None, None, None, None, None, None);
    }

    assert_shift!(shr_exact, T, T::MIN, 0);
    assert_shift!(shr_exact, T, T::MAX, None);
  });

  test!(@uint, test_next_multiple_of, () => {
    assert_eq!(T::P_1.next_multiple_of(T::P_2), T::P_2);
    assert_eq!(T::P_2.next_multiple_of(T::P_2), T::P_2);
    assert_eq!(T::MIN.next_multiple_of(T::P_2), T::MIN);
    assert_panic!(T::MAX.next_multiple_of(T::P_2), message = ADD);

    assert_panic!(T::P_1.next_multiple_of(T::MIN), message = REM_ZERO);
    assert_panic!(T::P_2.next_multiple_of(T::MIN), message = REM_ZERO);
    assert_panic!(T::MIN.next_multiple_of(T::MIN), message = REM_ZERO);
    assert_panic!(T::MAX.next_multiple_of(T::MIN), message = REM_ZERO);
  });

  test!(@uint, test_pow, () => {
    assert_eq!(T::P_1.pow(2), T::P_1);
    assert_eq!(T::P_2.pow(2), T::P_2 + T::P_2);
    assert_eq!(T::MIN.pow(2), T::MIN);
    assert_panic!(T::MAX.pow(2), message = MUL);
  });

  test!(#[no_miri] @uint, test_ilog, () => {
    assert_panic!(T::P_1.ilog(T::P_0), message = ILOG_BASE);
    assert_panic!(T::P_1.ilog(T::P_1), message = ILOG_BASE);
    assert_panic!(T::P_0.ilog(T::P_13), message = ILOG);

    for value in T::P_1..=T::P_255 {
      assert_ilog!(ilog(T::P_13), T, value);
    }

    for value in T::iter() {
      assert_ilog!(ilog(T::P_13), T, value);
    }
  });

  test!(#[no_miri] @uint, test_ilog2, () => {
    assert_panic!(T::P_0.ilog2(), message = ILOG);

    for value in T::P_1..=T::P_255 {
      assert_ilog!(ilog2, T, value);
    }

    for value in T::iter() {
      assert_ilog!(ilog2, T, value);
    }
  });

  test!(#[no_miri] @uint, test_ilog10, () => {
    assert_panic!(T::P_0.ilog10(), message = ILOG);

    for value in T::P_1..=T::P_255 {
      assert_ilog!(ilog10, T, value);
    }

    for value in T::iter() {
      assert_ilog!(ilog10, T, value);
    }
  });

  test!(#[no_miri] @uint, test_isqrt, () => {
    assert_isqrt!(isqrt, T, T::P_0);

    for value in T::P_1..=T::P_255 {
      assert_isqrt!(isqrt, T, value);
    }

    for value in T::iter() {
      assert_isqrt!(@call isqrt, T, value);
    }
  });

  test!(@uint, test_abs_diff, () => {
    assert_eq!(T::P_1.abs_diff(T::MIN), T::P_1);
    assert_eq!(T::P_2.abs_diff(T::MIN), T::P_2);
    assert_eq!(T::MIN.abs_diff(T::MIN), T::MIN);
    assert_eq!(T::MAX.abs_diff(T::MIN), T::MAX);

    assert_eq!(T::P_1.abs_diff(T::MAX), T::MAX - T::P_1);
    assert_eq!(T::P_2.abs_diff(T::MAX), T::MAX - T::P_2);
    assert_eq!(T::MIN.abs_diff(T::MAX), T::MAX);
    assert_eq!(T::MAX.abs_diff(T::MAX), T::MIN);
  });

  test!(@uint, test_next_power_of_two, () => {
    assert_eq!(T::P_1.next_power_of_two(), T::P_1);
    assert_eq!(T::P_2.next_power_of_two(), T::P_2);
    assert_eq!(T::MIN.next_power_of_two(), T::P_1);
    assert_panic!(T::MAX.next_power_of_two(), message = ADD);
  });

  test!(@uint, test_is_power_of_two, () => {
    assert_eq!(T::P_1.is_power_of_two(), true);
    assert_eq!(T::P_2.is_power_of_two(), true);
    assert_eq!(T::MIN.is_power_of_two(), false);
    assert_eq!(T::MAX.is_power_of_two(), false);
  });

  test!(@uint, test_is_multiple_of, () => {
    assert_eq!(T::P_1.is_multiple_of(T::MIN), false);
    assert_eq!(T::P_2.is_multiple_of(T::MIN), false);
    assert_eq!(T::MIN.is_multiple_of(T::MIN), true);
    assert_eq!(T::MAX.is_multiple_of(T::MIN), false);

    assert_eq!(T::P_1.is_multiple_of(T::P_2), false);
    assert_eq!(T::P_2.is_multiple_of(T::P_2), true);
    assert_eq!(T::MIN.is_multiple_of(T::P_2), true);
    assert_eq!(T::MAX.is_multiple_of(T::P_2), false);
  });

  test!(@uint, test_funnel_shl, () => {
    assert_eq!(T::P_1.funnel_shl(T::P_1, 0), T::P_1);
    assert_eq!(T::P_2.funnel_shl(T::P_1, 0), T::P_2);
    assert_eq!(T::MIN.funnel_shl(T::P_1, 0), T::MIN);
    assert_eq!(T::MAX.funnel_shl(T::P_1, 0), T::MAX);

    assert_eq!(T::P_1.funnel_shl(T::MIN, 4), T::P_1 << 4);
    assert_eq!(T::P_2.funnel_shl(T::MIN, 4), T::P_2 << 4);
    assert_eq!(T::MIN.funnel_shl(T::MIN, 4), T::MIN << 4);
    assert_eq!(T::MAX.funnel_shl(T::MIN, 4), T::MAX << 4);

    assert_eq!(T::P_1.funnel_shl(T::P_1, 4), T::P_1.rotate_left(4));
    assert_eq!(T::P_2.funnel_shl(T::P_2, 4), T::P_2.rotate_left(4));
    assert_eq!(T::MIN.funnel_shl(T::MIN, 4), T::MIN.rotate_left(4));
    assert_eq!(T::MAX.funnel_shl(T::MAX, 4), T::MAX.rotate_left(4));

    assert_panic!(T::P_1.funnel_shl(T::MIN, T::BITS + 1), message = FSHL);
    assert_panic!(T::P_2.funnel_shl(T::MIN, T::BITS + 1), message = FSHL);
    assert_panic!(T::MIN.funnel_shl(T::MIN, T::BITS + 1), message = FSHL);
    assert_panic!(T::MAX.funnel_shl(T::MIN, T::BITS + 1), message = FSHL);
  });

  test!(@uint, test_funnel_shr, () => {
    assert_eq!(T::P_1.funnel_shr(T::P_1, 0), T::P_1);
    assert_eq!(T::P_2.funnel_shr(T::P_1, 0), T::P_1);
    assert_eq!(T::MIN.funnel_shr(T::P_1, 0), T::P_1);
    assert_eq!(T::MAX.funnel_shr(T::P_1, 0), T::P_1);

    assert_eq!(T::MIN.funnel_shr(T::P_1, 4), T::P_1 >> 4);
    assert_eq!(T::MIN.funnel_shr(T::P_2, 4), T::P_2 >> 4);
    assert_eq!(T::MIN.funnel_shr(T::MIN, 4), T::MIN >> 4);
    assert_eq!(T::MIN.funnel_shr(T::MAX, 4), T::MAX >> 4);

    assert_eq!(T::P_1.funnel_shr(T::P_1, 4), T::P_1.rotate_right(4));
    assert_eq!(T::P_2.funnel_shr(T::P_2, 4), T::P_2.rotate_right(4));
    assert_eq!(T::MIN.funnel_shr(T::MIN, 4), T::MIN.rotate_right(4));
    assert_eq!(T::MAX.funnel_shr(T::MAX, 4), T::MAX.rotate_right(4));

    assert_panic!(T::P_1.funnel_shr(T::MIN, T::BITS + 1), message = FSHR);
    assert_panic!(T::P_2.funnel_shr(T::MIN, T::BITS + 1), message = FSHR);
    assert_panic!(T::MIN.funnel_shr(T::MIN, T::BITS + 1), message = FSHR);
    assert_panic!(T::MAX.funnel_shr(T::MIN, T::BITS + 1), message = FSHR);
  });
}
