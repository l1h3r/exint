use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::isqrt;
use crate::llapi;
use crate::logxx;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "checked_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_add(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add_unsigned(self, rhs: uint<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_add_unsigned(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_sub(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub_unsigned(self, rhs: uint<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_sub_unsigned(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_mul(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
      Some(unsafe { llapi::unchecked_sdiv::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(int, "checked_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      Some(self.div_euclid(rhs))
    }
  }

  #[doc = include_doc!(int, "checked_div_exact")]
  #[cfg(feature = "exact_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div_exact(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
      unsafe {
        if llapi::unlikely(!llapi::unchecked_srem::<Self, N>(self, rhs).is_zero()) {
          None
        } else {
          Some(self.unchecked_div_exact(rhs))
        }
      }
    }
  }

  #[doc = include_doc!(int, "checked_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) && rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero or Self::MIN / -1.
      Some(unsafe { llapi::unchecked_srem::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(int, "checked_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero() || (self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE))) {
      None
    } else {
      Some(self.rem_euclid(rhs))
    }
  }

  #[doc = include_doc!(int, "checked_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shl(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(int, "checked_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      Some(unsafe { self.unchecked_shr(rhs) })
    } else {
      None
    }
  }

  #[doc = include_doc!(int, "checked_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_neg(self) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_neg();

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(int, "checked_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_pow(self, mut exp: u32) -> Option<Self> {
    if exp == 0 {
      return Some(Self::ONE);
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;

    loop {
      if (exp & 1) == 1 {
        acc = tri!(acc.checked_mul(base));

        if exp == 1 {
          return Some(acc);
        }
      }

      exp /= 2;
      base = tri!(base.checked_mul(base));
    }
  }

  #[doc = include_doc!(int, "checked_next_multiple_of")]
  #[cfg(feature = "int_roundings")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
    if rhs.const_eq(&Self::NEG_ONE) {
      return Some(self);
    }

    let mut value: Self = tri!(self.checked_rem(rhs));

    if (value.is_positive() && rhs.is_negative()) || (value.is_negative() && rhs.is_positive()) {
      value = value.const_add(rhs);
    }

    if value.is_zero() {
      Some(self)
    } else {
      self.checked_add(rhs.const_sub(value))
    }
  }

  #[doc = include_doc!(int, "checked_ilog")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog(self, base: Self) -> Option<u32> {
    if self.const_le(&Self::ZERO) || base.const_le(&Self::ONE) {
      None
    } else {
      self.cast_unsigned().checked_ilog(base.cast_unsigned())
    }
  }

  #[doc = include_doc!(int, "checked_ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog2(self) -> Option<u32> {
    if self.is_positive() {
      // SAFETY: We just ensured that `self` is positive.
      Some(unsafe { logxx::ilog2::<N>(self.cast_unsigned()) })
    } else {
      None
    }
  }

  #[doc = include_doc!(int, "checked_ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog10(self) -> Option<u32> {
    if self.is_positive() {
      // SAFETY: We just ensured that `self` is positive.
      Some(unsafe { logxx::ilog10::<N>(self.cast_unsigned()) })
    } else {
      None
    }
  }

  #[doc = include_doc!(int, "checked_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_abs(self) -> Option<Self> {
    if self.is_negative() {
      self.checked_neg()
    } else {
      Some(self)
    }
  }

  #[doc = include_doc!(int, "checked_isqrt")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_isqrt(self) -> Option<Self> {
    if self.is_negative() {
      None
    } else {
      // SAFETY: We just ensured that `self` is non-negative.
      let result: Self = unsafe { isqrt::int_sqrt(self) };

      // SAFETY: Integer square root is a monotonically nondecreasing function,
      // which means that increasing the input will never decrease the output.
      unsafe {
        let max: Self = const { isqrt::int_sqrt(Self::MAX) };

        ::core::hint::assert_unchecked(result.const_ge(&Self::ZERO));
        ::core::hint::assert_unchecked(result.const_le(&max));
      }

      Some(result)
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_checked_add, () => {
    assert_eq!(T::MIN.checked_add(T::N_1), None);
    assert_eq!(T::MIN.checked_add(T::P_1), Some(T::MIN + T::P_1));
    assert_eq!(T::MIN.checked_add(T::MIN), None);
    assert_eq!(T::MIN.checked_add(T::MAX), Some(T::N_1));

    assert_eq!(T::MAX.checked_add(T::N_1), Some(T::MAX - T::P_1));
    assert_eq!(T::MAX.checked_add(T::P_1), None);
    assert_eq!(T::MAX.checked_add(T::MIN), Some(T::N_1));
    assert_eq!(T::MAX.checked_add(T::MAX), None);
  });

  test!(@sint, test_checked_add_unsigned, () => {
    assert_eq!(T::MIN.checked_add_unsigned(T::N_1.cast_unsigned()), Some(T::MAX));
    assert_eq!(T::MIN.checked_add_unsigned(T::P_1.cast_unsigned()), Some(T::MIN + T::P_1));
    assert_eq!(T::MIN.checked_add_unsigned(T::MIN.cast_unsigned()), Some(T::P_0));
    assert_eq!(T::MIN.checked_add_unsigned(T::MAX.cast_unsigned()), Some(T::N_1));

    assert_eq!(T::MAX.checked_add_unsigned(T::N_1.cast_unsigned()), None);
    assert_eq!(T::MAX.checked_add_unsigned(T::P_1.cast_unsigned()), None);
    assert_eq!(T::MAX.checked_add_unsigned(T::MIN.cast_unsigned()), None);
    assert_eq!(T::MAX.checked_add_unsigned(T::MAX.cast_unsigned()), None);
  });

  test!(@sint, test_checked_sub, () => {
    assert_eq!(T::MIN.checked_sub(T::N_1), Some(T::MIN + T::P_1));
    assert_eq!(T::MIN.checked_sub(T::P_1), None);
    assert_eq!(T::MIN.checked_sub(T::MIN), Some(T::P_0));
    assert_eq!(T::MIN.checked_sub(T::MAX), None);

    assert_eq!(T::MAX.checked_sub(T::N_1), None);
    assert_eq!(T::MAX.checked_sub(T::P_1), Some(T::MAX - T::P_1));
    assert_eq!(T::MAX.checked_sub(T::MIN), None);
    assert_eq!(T::MAX.checked_sub(T::MAX), Some(T::P_0));
  });

  test!(@sint, test_checked_sub_unsigned, () => {
    assert_eq!(T::MIN.checked_sub_unsigned(T::N_1.cast_unsigned()), None);
    assert_eq!(T::MIN.checked_sub_unsigned(T::P_1.cast_unsigned()), None);
    assert_eq!(T::MIN.checked_sub_unsigned(T::MIN.cast_unsigned()), None);
    assert_eq!(T::MIN.checked_sub_unsigned(T::MAX.cast_unsigned()), None);

    assert_eq!(T::MAX.checked_sub_unsigned(T::N_1.cast_unsigned()), Some(T::MIN));
    assert_eq!(T::MAX.checked_sub_unsigned(T::P_1.cast_unsigned()), Some(T::MAX - T::P_1));
    assert_eq!(T::MAX.checked_sub_unsigned(T::MIN.cast_unsigned()), Some(T::N_1));
    assert_eq!(T::MAX.checked_sub_unsigned(T::MAX.cast_unsigned()), Some(T::P_0));
  });

  test!(@sint, test_checked_mul, () => {
    assert_eq!(T::MIN.checked_mul(T::N_1), None);
    assert_eq!(T::MIN.checked_mul(T::P_1), Some(T::MIN));
    assert_eq!(T::MIN.checked_mul(T::MIN), None);
    assert_eq!(T::MIN.checked_mul(T::MAX), None);

    assert_eq!(T::MAX.checked_mul(T::N_1), Some(-T::MAX));
    assert_eq!(T::MAX.checked_mul(T::P_1), Some(T::MAX));
    assert_eq!(T::MAX.checked_mul(T::MIN), None);
    assert_eq!(T::MAX.checked_mul(T::MAX), None);
  });

  test!(@sint, test_checked_div, () => {
    assert_eq!(T::N_1.checked_div(T::N_1), Some(T::P_1));
    assert_eq!(T::P_0.checked_div(T::N_1), Some(T::P_0));
    assert_eq!(T::P_1.checked_div(T::N_1), Some(T::N_1));
    assert_eq!(T::MIN.checked_div(T::N_1), None);
    assert_eq!(T::MAX.checked_div(T::N_1), Some(T::MIN + T::P_1));

    assert_eq!(T::N_1.checked_div(T::P_0), None);
    assert_eq!(T::P_0.checked_div(T::P_0), None);
    assert_eq!(T::P_1.checked_div(T::P_0), None);
    assert_eq!(T::MIN.checked_div(T::P_0), None);
    assert_eq!(T::MAX.checked_div(T::P_0), None);
  });

  test!(@sint, test_checked_div_euclid, () => {
    assert_eq!(T::N_1.checked_div_euclid(T::N_1), Some(T::P_1));
    assert_eq!(T::P_0.checked_div_euclid(T::N_1), Some(T::P_0));
    assert_eq!(T::P_1.checked_div_euclid(T::N_1), Some(T::N_1));
    assert_eq!(T::MIN.checked_div_euclid(T::N_1), None);
    assert_eq!(T::MAX.checked_div_euclid(T::N_1), Some(T::MIN + T::P_1));

    assert_eq!(T::N_1.checked_div_euclid(T::P_0), None);
    assert_eq!(T::P_0.checked_div_euclid(T::P_0), None);
    assert_eq!(T::P_1.checked_div_euclid(T::P_0), None);
    assert_eq!(T::MIN.checked_div_euclid(T::P_0), None);
    assert_eq!(T::MAX.checked_div_euclid(T::P_0), None);
  });

  test!(@sint, test_checked_div_exact, () => {
    assert_eq!(T::N_1.checked_div_exact(T::N_1), Some(T::P_1));
    assert_eq!(T::P_1.checked_div_exact(T::N_1), Some(T::N_1));
    assert_eq!(T::P_2.checked_div_exact(T::N_1), Some(T::N_2));
    assert_eq!(T::MIN.checked_div_exact(T::N_1), None);
    assert_eq!(T::MAX.checked_div_exact(T::N_1), Some(T::MIN + T::P_1));

    assert_eq!(T::N_1.checked_div_exact(T::P_0), None);
    assert_eq!(T::P_1.checked_div_exact(T::P_0), None);
    assert_eq!(T::P_2.checked_div_exact(T::P_0), None);
    assert_eq!(T::MIN.checked_div_exact(T::P_0), None);
    assert_eq!(T::MAX.checked_div_exact(T::P_0), None);
  });

  test!(@sint, test_checked_rem, () => {
    assert_eq!(T::N_1.checked_rem(T::N_1), Some(T::P_0));
    assert_eq!(T::P_0.checked_rem(T::N_1), Some(T::P_0));
    assert_eq!(T::P_1.checked_rem(T::N_1), Some(T::P_0));
    assert_eq!(T::MIN.checked_rem(T::N_1), None);
    assert_eq!(T::MAX.checked_rem(T::N_1), Some(T::P_0));

    assert_eq!(T::N_1.checked_rem(T::P_0), None);
    assert_eq!(T::P_0.checked_rem(T::P_0), None);
    assert_eq!(T::P_1.checked_rem(T::P_0), None);
    assert_eq!(T::MIN.checked_rem(T::P_0), None);
    assert_eq!(T::MAX.checked_rem(T::P_0), None);
  });

  test!(@sint, test_checked_rem_euclid, () => {
    assert_eq!(T::N_1.checked_rem_euclid(T::N_1), Some(T::P_0));
    assert_eq!(T::P_0.checked_rem_euclid(T::N_1), Some(T::P_0));
    assert_eq!(T::P_1.checked_rem_euclid(T::N_1), Some(T::P_0));
    assert_eq!(T::MIN.checked_rem_euclid(T::N_1), None);
    assert_eq!(T::MAX.checked_rem_euclid(T::N_1), Some(T::P_0));

    assert_eq!(T::N_1.checked_rem_euclid(T::P_0), None);
    assert_eq!(T::P_0.checked_rem_euclid(T::P_0), None);
    assert_eq!(T::P_1.checked_rem_euclid(T::P_0), None);
    assert_eq!(T::MIN.checked_rem_euclid(T::P_0), None);
    assert_eq!(T::MAX.checked_rem_euclid(T::P_0), None);
  });

  test!(@sint, test_checked_shl, () => {
    assert_checked_shift!(checked_shl, T, T::N_1);
    assert_checked_shift!(checked_shl, T, T::P_1);
    assert_checked_shift!(checked_shl, T, T::P_8);
    assert_checked_shift!(checked_shl, T, T::P_17);
    assert_checked_shift!(checked_shl, T, T::MIN);
    assert_checked_shift!(checked_shl, T, T::MAX);
  });

  test!(@sint, test_checked_shr, () => {
    assert_checked_shift!(checked_shr, T, T::N_1);
    assert_checked_shift!(checked_shr, T, T::P_1);
    assert_checked_shift!(checked_shr, T, T::P_8);
    assert_checked_shift!(checked_shr, T, T::P_17);
    assert_checked_shift!(checked_shr, T, T::MIN);
    assert_checked_shift!(checked_shr, T, T::MAX);
  });

  test!(@sint, test_checked_neg, () => {
    assert_eq!(T::N_1.checked_neg(), Some(T::P_1));
    assert_eq!(T::P_0.checked_neg(), Some(T::P_0));
    assert_eq!(T::P_1.checked_neg(), Some(T::N_1));
    assert_eq!(T::MIN.checked_neg(), None);
    assert_eq!(T::MAX.checked_neg(), Some(T::MIN + T::P_1));
  });

  test!(@sint, test_checked_pow, () => {
    assert_eq!(T::N_1.checked_pow(2), Some(T::P_1));
    assert_eq!(T::P_0.checked_pow(2), Some(T::P_0));
    assert_eq!(T::P_1.checked_pow(2), Some(T::P_1));
    assert_eq!(T::MIN.checked_pow(2), None);
    assert_eq!(T::MAX.checked_pow(2), None);
  });

  test!(@sint, test_checked_next_multiple_of, () => {
    assert_eq!(T::N_1.checked_next_multiple_of(T::P_2), Some(T::P_0));
    assert_eq!(T::P_0.checked_next_multiple_of(T::P_2), Some(T::P_0));
    assert_eq!(T::P_1.checked_next_multiple_of(T::P_2), Some(T::P_2));
    assert_eq!(T::MIN.checked_next_multiple_of(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_next_multiple_of(T::P_2), None);

    assert_eq!(T::N_1.checked_next_multiple_of(T::P_0), None);
    assert_eq!(T::P_0.checked_next_multiple_of(T::P_0), None);
    assert_eq!(T::P_1.checked_next_multiple_of(T::P_0), None);
    assert_eq!(T::MIN.checked_next_multiple_of(T::P_0), None);
    assert_eq!(T::MAX.checked_next_multiple_of(T::P_0), None);
  });

  test!(#[no_miri] @sint, test_checked_ilog, () => {
    assert_eq!(T::P_1.checked_ilog(T::P_0), None);
    assert_eq!(T::P_1.checked_ilog(T::P_1), None);

    for value in T::N_128..=T::P_0 {
      assert_eq!(value.checked_ilog(T::P_13), None);
    }

    for value in T::P_1..=T::P_127 {
      assert_ilog!(checked_ilog(T::P_13), T, value);
    }

    for value in T::iter() {
      assert_ilog!(checked_ilog(T::P_13), T, value);
    }
  });

  test!(#[no_miri] @sint, test_checked_ilog2, () => {
    for value in T::N_128..=T::P_0 {
      assert_eq!(value.checked_ilog2(), None);
    }

    for value in T::P_1..=T::P_127 {
      assert_ilog!(checked_ilog2, T, value);
    }

    for value in T::iter() {
      assert_ilog!(checked_ilog2, T, value);
    }
  });

  test!(#[no_miri] @sint, test_checked_ilog10, () => {
    for value in T::N_128..=T::P_0 {
      assert_eq!(value.checked_ilog10(), None);
    }

    for value in T::P_1..=T::P_127 {
      assert_ilog!(checked_ilog10, T, value);
    }

    for value in T::iter() {
      assert_ilog!(checked_ilog10, T, value);
    }
  });

  test!(@sint, test_checked_abs, () => {
    assert_eq!(T::N_1.checked_abs(), Some(T::P_1));
    assert_eq!(T::P_0.checked_abs(), Some(T::P_0));
    assert_eq!(T::P_1.checked_abs(), Some(T::P_1));
    assert_eq!(T::MIN.checked_abs(), None);
    assert_eq!(T::MAX.checked_abs(), Some(T::MAX));
  });

  test!(@sint, test_checked_isqrt, () => {
    assert_isqrt!(checked_isqrt, T, T::N_1, None);
    assert_isqrt!(checked_isqrt, T, T::P_0);
    assert_isqrt!(checked_isqrt, T, T::P_1);
    assert_isqrt!(checked_isqrt, T, T::MIN, None);
    assert_isqrt!(checked_isqrt, T, T::MAX);
  });
}
