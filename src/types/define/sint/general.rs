#![allow(unused_imports, reason = "feature-dependant")]

use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::llapi;
use crate::panic;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "max_value")]
  #[deprecated(
    since = "TBD",
    note = "replaced by the `MAX` associated constant on this type"
  )]
  #[must_use]
  #[inline]
  pub const fn max_value() -> Self {
    Self::MAX
  }

  #[doc = include_doc!(int, "min_value")]
  #[deprecated(
    since = "TBD",
    note = "replaced by the `MIN` associated constant on this type"
  )]
  #[must_use]
  #[inline]
  pub const fn min_value() -> Self {
    Self::MIN
  }

  #[doc = include_doc!(int, "cast_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn cast_unsigned(self) -> uint<N> {
    uint::from_ne_bytes(self.to_ne_bytes())
  }

  #[doc = include_doc!(int, "midpoint")]
  #[doc(alias = "average_floor")]
  #[doc(alias = "average_ceil")]
  #[doc(alias = "average")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn midpoint(self, rhs: Self) -> Self {
    let out: Self = self
      .const_bxor(rhs)
      .const_shr(1)
      .const_add(self.const_band(rhs));

    let add: Self = if out.is_negative() {
      Self::ONE.const_band(self.const_bxor(rhs))
    } else {
      Self::ZERO.const_band(self.const_bxor(rhs))
    };

    out.const_add(add)
  }

  #[doc = include_doc!(int, "div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_euclid(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if rem.is_negative() {
      if rhs.is_positive() {
        div.const_sub(Self::ONE)
      } else {
        div.const_add(Self::ONE)
      }
    } else {
      div
    }
  }

  #[doc = include_doc!(int, "rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn rem_euclid(self, rhs: Self) -> Self {
    let rem: Self = self.const_rem(rhs);

    if rem.is_negative() {
      rem.wrapping_add(rhs.wrapping_abs())
    } else {
      rem
    }
  }

  #[doc = include_doc!(int, "div_ceil")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_ceil(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if llapi::unlikely(rem.is_zero()) {
      div
    } else {
      self
        .const_bxor(rhs)
        .const_shr(Self::BITS - 1)
        .const_add(Self::ONE)
        .const_add(div)
    }
  }

  #[doc = include_doc!(int, "div_floor")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn div_floor(self, rhs: Self) -> Self {
    let div: Self = self.const_div(rhs);
    let rem: Self = self.const_rem(rhs);

    if llapi::unlikely(rem.is_zero()) {
      div
    } else {
      self
        .const_bxor(rhs)
        .const_shr(Self::BITS - 1)
        .const_add(div)
    }
  }

  #[doc = include_doc!(int, "exact_div")]
  #[cfg(feature = "exact_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn exact_div(self, rhs: Self) -> Self {
    match self.checked_exact_div(rhs) {
      Some(value) => value,
      None => panic::exact_div(),
    }
  }

  #[doc = include_doc!(int, "exact_shl")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn exact_shl(self, rhs: u32) -> Option<Self> {
    if rhs < self.leading_zeros() || rhs < self.leading_ones() {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shl(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(int, "exact_shr")]
  #[cfg(feature = "exact_bitshifts")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn exact_shr(self, rhs: u32) -> Option<Self> {
    if rhs <= self.trailing_zeros() && rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shr(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(int, "next_multiple_of")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn next_multiple_of(self, rhs: Self) -> Self {
    if rhs.const_eq(&Self::NEG_ONE) {
      return self;
    }

    let mut rem: Self = self.const_rem(rhs);

    if (rem.is_positive() && rhs.is_negative()) || (rem.is_negative() && rhs.is_positive()) {
      rem = rem.const_add(rhs);
    }

    if rem.is_zero() {
      self
    } else {
      self.const_add(rhs.const_sub(rem))
    }
  }

  #[doc = include_doc!(int, "pow")]
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

  #[doc = include_doc!(int, "ilog")]
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

  #[doc = include_doc!(int, "ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog2(self) -> u32 {
    match self.checked_ilog2() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(int, "ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn ilog10(self) -> u32 {
    match self.checked_ilog10() {
      Some(log) => log,
      None => panic::ilog(),
    }
  }

  #[doc = include_doc!(int, "isqrt")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn isqrt(self) -> Self {
    match self.checked_isqrt() {
      Some(sqrt) => sqrt,
      None => panic::isqrt(),
    }
  }

  #[doc = include_doc!(int, "abs_diff")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs_diff(self, rhs: Self) -> uint<N> {
    if self.const_lt(&rhs) {
      rhs.cast_unsigned().wrapping_sub(self.cast_unsigned())
    } else {
      self.cast_unsigned().wrapping_sub(rhs.cast_unsigned())
    }
  }

  #[doc = include_doc!(int, "abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn abs(self) -> Self {
    if self.is_negative() {
      self.const_neg()
    } else {
      self
    }
  }

  #[doc = include_doc!(int, "unsigned_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unsigned_abs(self) -> uint<N> {
    self.wrapping_abs().cast_unsigned()
  }

  #[doc = include_doc!(int, "is_negative")]
  #[must_use]
  #[inline]
  pub const fn is_negative(self) -> bool {
    self.const_lt(&Self::ZERO)
  }

  #[doc = include_doc!(int, "is_positive")]
  #[must_use]
  #[inline]
  pub const fn is_positive(self) -> bool {
    self.const_gt(&Self::ZERO)
  }

  #[doc = include_doc!(int, "signum")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn signum(self) -> Self {
    Self::from_i8(self.const_cmp(&Self::ZERO) as i8)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_max_value, () => {
    assert_eq!(T::max_value(), T::MAX);
  });

  test!(@sint, test_min_value, () => {
    assert_eq!(T::min_value(), T::MIN);
  });

  test!(@sint, test_cast_unsigned, () => {
    assert_eq!(T::N_1.cast_unsigned(), U::MAX);
    assert_eq!(T::P_0.cast_unsigned(), U::MIN);
    assert_eq!(T::P_1.cast_unsigned(), U::from_u8(1));
    assert_eq!(T::MIN.cast_unsigned(), U::MIN | (U::from_u8(1) << (T::BITS - 1)));
    assert_eq!(T::MAX.cast_unsigned(), U::MAX ^ (U::from_u8(1) << (T::BITS - 1)));
  });

  test!(@sint, test_midpoint, () => {
    assert_eq!(T::MIN.midpoint(T::N_1), T::MIN / T::P_2);
    assert_eq!(T::MIN.midpoint(T::P_1), T::MIN / T::P_2 + T::P_1);
    assert_eq!(T::MIN.midpoint(T::MIN), T::MIN);
    assert_eq!(T::MIN.midpoint(T::MAX), T::P_0);

    assert_eq!(T::MAX.midpoint(T::N_1), T::MAX / T::P_2);
    assert_eq!(T::MAX.midpoint(T::P_1), T::MAX / T::P_2 + T::P_1);
    assert_eq!(T::MAX.midpoint(T::MIN), T::P_0);
    assert_eq!(T::MAX.midpoint(T::MAX), T::MAX);
  });

  test!(@sint, test_div_euclid, () => {
    assert_eq!(T::N_1.div_euclid(T::N_1), T::P_1);
    assert_eq!(T::P_0.div_euclid(T::N_1), T::P_0);
    assert_eq!(T::P_1.div_euclid(T::N_1), T::N_1);
    assert_panic!(T::MIN.div_euclid(T::N_1), message = DIV);
    assert_eq!(T::MAX.div_euclid(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.div_euclid(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_rem_euclid, () => {
    assert_eq!(T::N_1.rem_euclid(T::N_1), T::P_0);
    assert_eq!(T::P_0.rem_euclid(T::N_1), T::P_0);
    assert_eq!(T::P_1.rem_euclid(T::N_1), T::P_0);
    assert_panic!(T::MIN.rem_euclid(T::N_1), message = REM);
    assert_eq!(T::MAX.rem_euclid(T::N_1), T::P_0);

    assert_panic!(T::N_1.rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_0.rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_1.rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::MIN.rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::MAX.rem_euclid(T::P_0), message = REM_ZERO);
  });

  test!(@sint, test_div_ceil, () => {
    assert_eq!(T::N_1.div_ceil(T::N_1), T::P_1);
    assert_eq!(T::P_0.div_ceil(T::N_1), T::P_0);
    assert_eq!(T::P_1.div_ceil(T::N_1), T::N_1);
    assert_panic!(T::MIN.div_ceil(T::N_1), message = DIV);
    assert_eq!(T::MAX.div_ceil(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.div_ceil(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.div_ceil(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.div_ceil(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.div_ceil(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.div_ceil(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_div_floor, () => {
    assert_eq!(T::N_1.div_floor(T::N_1), T::P_1);
    assert_eq!(T::P_0.div_floor(T::N_1), T::P_0);
    assert_eq!(T::P_1.div_floor(T::N_1), T::N_1);
    assert_panic!(T::MIN.div_floor(T::N_1), message = DIV);
    assert_eq!(T::MAX.div_floor(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.div_floor(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.div_floor(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.div_floor(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.div_floor(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.div_floor(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_exact_div, () => {
    assert_eq!(T::N_1.exact_div(T::N_1), T::P_1);
    assert_eq!(T::P_1.exact_div(T::N_1), T::N_1);
    assert_eq!(T::P_2.exact_div(T::N_1), T::N_2);
    assert_panic!(T::MIN.exact_div(T::N_1), message = EXACT_DIV);
    assert_eq!(T::MAX.exact_div(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.exact_div(T::P_0), message = EXACT_DIV);
    assert_panic!(T::P_1.exact_div(T::P_0), message = EXACT_DIV);
    assert_panic!(T::P_2.exact_div(T::P_0), message = EXACT_DIV);
    assert_panic!(T::MIN.exact_div(T::P_0), message = EXACT_DIV);
    assert_panic!(T::MAX.exact_div(T::P_0), message = EXACT_DIV);
  });

  test!(@sint, test_exact_shl, () => {
    assert_shift!(exact_shl, T, T::N_1, 1, 3, 5, S_1, S_2, S_3, S_4);
    assert_shift!(exact_shl, T, T::P_0, P_0);
    assert_shift!(exact_shl, T, T::P_1, 1, 3, 5, None, S_2, S_3, S_4);
    assert_shift!(exact_shl, T, T::MIN, None);
    assert_shift!(exact_shl, T, T::MAX, None);
  });

  test!(@sint, test_exact_shr, () => {
    assert_shift!(exact_shr, T, T::N_1, None);
    assert_shift!(exact_shr, T, T::P_0, P_0);
    assert_shift!(exact_shr, T, T::P_1, None);
    assert_shift!(exact_shr, T, T::MIN, 1, 3, 5, S_1, S_2, S_3, S_4);
    assert_shift!(exact_shr, T, T::MAX, None);
  });

  test!(@sint, test_next_multiple_of, () => {
    assert_eq!(T::N_1.next_multiple_of(T::P_2), T::P_0);
    assert_eq!(T::P_0.next_multiple_of(T::P_2), T::P_0);
    assert_eq!(T::P_1.next_multiple_of(T::P_2), T::P_2);
    assert_eq!(T::MIN.next_multiple_of(T::P_2), T::MIN);
    assert_panic!(T::MAX.next_multiple_of(T::P_2), message = ADD);

    assert_panic!(T::N_1.next_multiple_of(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_0.next_multiple_of(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_1.next_multiple_of(T::P_0), message = REM_ZERO);
    assert_panic!(T::MIN.next_multiple_of(T::P_0), message = REM_ZERO);
    assert_panic!(T::MAX.next_multiple_of(T::P_0), message = REM_ZERO);
  });

  test!(@sint, test_pow, () => {
    assert_eq!(T::N_1.pow(2), T::P_1);
    assert_eq!(T::P_0.pow(2), T::P_0);
    assert_eq!(T::P_1.pow(2), T::P_1);
    assert_panic!(T::MIN.pow(2), message = MUL);
    assert_panic!(T::MAX.pow(2), message = MUL);
  });

  test!(#[no_miri] @sint, test_ilog, () => {
    assert_panic!(T::P_1.ilog(T::P_0), message = ILOG_BASE);
    assert_panic!(T::P_1.ilog(T::P_1), message = ILOG_BASE);

    for value in T::N_128..=T::P_0 {
      assert_panic!(value.ilog(T::P_13), message = ILOG);
    }

    for value in T::P_1..=T::P_127 {
      assert_ilog!(ilog(T::P_13), T, value);
    }

    for value in T::iter() {
      assert_ilog!(ilog(T::P_13), T, value);
    }
  });

  test!(#[no_miri] @sint, test_ilog2, () => {
    for value in T::N_128..=T::P_0 {
      assert_panic!(value.ilog2(), message = ILOG);
    }

    for value in T::P_1..=T::P_127 {
      assert_ilog!(ilog2, T, value);
    }

    for value in T::iter() {
      assert_ilog!(ilog2, T, value);
    }
  });

  test!(#[no_miri] @sint, test_ilog10, () => {
    for value in T::N_128..=T::P_0 {
      assert_panic!(value.ilog10(), message = ILOG);
    }

    for value in T::P_1..=T::P_127 {
      assert_ilog!(ilog10, T, value);
    }

    for value in T::iter() {
      assert_ilog!(ilog10, T, value);
    }
  });

  test!(@sint, test_isqrt, () => {
    assert_panic!(T::N_1.isqrt(), message = ISQRT);
    assert_isqrt!(isqrt, T, T::P_0);
    assert_isqrt!(isqrt, T, T::P_1);
    assert_panic!(T::MIN.isqrt(), message = ISQRT);
    assert_isqrt!(isqrt, T, T::MAX);
  });

  test!(@sint, test_abs_diff, () => {
    assert_eq!(T::N_1.abs_diff(T::MIN), T::MAX.cast_unsigned());
    assert_eq!(T::P_1.abs_diff(T::MIN), T::MIN.cast_unsigned() + U::from_u8(1));
    assert_eq!(T::MIN.abs_diff(T::MIN), U::MIN);
    assert_eq!(T::MAX.abs_diff(T::MIN), U::MAX);

    assert_eq!(T::N_1.abs_diff(T::MAX), T::MIN.cast_unsigned());
    assert_eq!(T::P_1.abs_diff(T::MAX), T::MAX.cast_unsigned() - U::from_u8(1));
    assert_eq!(T::MIN.abs_diff(T::MAX), U::MAX);
    assert_eq!(T::MAX.abs_diff(T::MAX), U::MIN);
  });

  test!(@sint, test_abs, () => {
    assert_eq!(T::N_1.abs(), T::P_1);
    assert_eq!(T::P_0.abs(), T::P_0);
    assert_eq!(T::P_1.abs(), T::P_1);
    assert_panic!(T::MIN.abs(), message = NEG);
    assert_eq!(T::MAX.abs(), T::MAX);
  });

  test!(@sint, test_unsigned_abs, () => {
    assert_eq!(T::N_1.unsigned_abs(), U::from_u8(1));
    assert_eq!(T::P_0.unsigned_abs(), U::MIN);
    assert_eq!(T::P_1.unsigned_abs(), U::from_u8(1));
    assert_eq!(T::MIN.unsigned_abs(), T::MIN.cast_unsigned());
    assert_eq!(T::MAX.unsigned_abs(), T::MAX.cast_unsigned());
  });

  test!(@sint, test_is_negative, () => {
    assert_eq!(T::N_1.is_negative(), true);
    assert_eq!(T::P_0.is_negative(), false);
    assert_eq!(T::P_1.is_negative(), false);
    assert_eq!(T::MIN.is_negative(), true);
    assert_eq!(T::MAX.is_negative(), false);
  });

  test!(@sint, test_is_positive, () => {
    assert_eq!(T::N_1.is_positive(), false);
    assert_eq!(T::P_0.is_positive(), false);
    assert_eq!(T::P_1.is_positive(), true);
    assert_eq!(T::MIN.is_positive(), false);
    assert_eq!(T::MAX.is_positive(), true);
  });

  test!(@sint, test_signum, () => {
    assert_eq!(T::N_1.signum(), T::N_1);
    assert_eq!(T::P_0.signum(), T::P_0);
    assert_eq!(T::P_1.signum(), T::P_1);
    assert_eq!(T::MIN.signum(), T::N_1);
    assert_eq!(T::MAX.signum(), T::P_1);
  });
}
