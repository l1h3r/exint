use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::llapi;
use crate::logxx;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "checked_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(llapi::overflowing_uadd::<Self, N>(self, rhs).1) {
      None
    } else {
      // SAFETY: We just ensured that this does not overflow.
      Some(unsafe { llapi::unchecked_uadd::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_add_signed(self, rhs: int<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_add_signed(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(uint, "checked_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    if self.const_lt(&rhs) {
      None
    } else {
      // SAFETY: We just ensured that this does not overflow.
      Some(unsafe { llapi::unchecked_usub::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_sub_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_sub_signed(self, rhs: int<N>) -> Option<Self> {
    let (result, overflow): (Self, bool) = self.overflowing_sub_signed(rhs);

    if llapi::unlikely(overflow) {
      None
    } else {
      Some(result)
    }
  }

  #[doc = include_doc!(uint, "checked_mul")]
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

  #[doc = include_doc!(uint, "checked_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero.
      Some(unsafe { llapi::unchecked_udiv::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      Some(self.div_euclid(rhs))
    }
  }

  #[doc = include_doc!(uint, "checked_div_exact")]
  #[cfg(feature = "exact_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_div_exact(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero.
      unsafe {
        if llapi::unlikely(!llapi::unchecked_urem::<Self, N>(self, rhs).is_zero()) {
          None
        } else {
          Some(self.unchecked_div_exact(rhs))
        }
      }
    }
  }

  #[doc = include_doc!(uint, "checked_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that we are not dividing by zero.
      Some(unsafe { llapi::unchecked_urem::<Self, N>(self, rhs) })
    }
  }

  #[doc = include_doc!(uint, "checked_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
    if llapi::unlikely(rhs.is_zero()) {
      None
    } else {
      Some(self.rem_euclid(rhs))
    }
  }

  #[doc = include_doc!(uint, "checked_shl")]
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

  #[doc = include_doc!(uint, "checked_shr")]
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

  #[doc = include_doc!(uint, "checked_neg")]
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

  #[doc = include_doc!(uint, "checked_pow")]
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

  #[doc = include_doc!(uint, "checked_next_power_of_two")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_next_power_of_two(self) -> Option<Self> {
    self
      .one_less_than_next_power_of_two()
      .checked_add(Self::ONE)
  }

  #[doc = include_doc!(uint, "checked_next_multiple_of")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
    match self.checked_rem(rhs) {
      Some(result) if result.is_zero() => Some(self),
      Some(result) => self.checked_add(rhs.const_sub(result)),
      None => None,
    }
  }

  #[doc = include_doc!(uint, "checked_ilog")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog(self, base: Self) -> Option<u32> {
    #[cfg(feature = "core_intrinsics")]
    if ::core::intrinsics::is_val_statically_known(base) {
      if base.const_eq(&Self::TWO) {
        return self.checked_ilog2();
      } else if base.const_eq(&Self::TEN) {
        return self.checked_ilog10();
      }
    }

    if self.const_le(&Self::ZERO) || base.const_le(&Self::ONE) {
      None
    } else {
      // SAFETY: We just ensured that `self` is not zero.
      Some(unsafe { logxx::ilog::<N>(self, base) })
    }
  }

  #[doc = include_doc!(uint, "checked_ilog2")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog2(self) -> Option<u32> {
    if llapi::unlikely(self.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that `self` is not zero.
      Some(unsafe { logxx::ilog2::<N>(self) })
    }
  }

  #[doc = include_doc!(uint, "checked_ilog10")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_ilog10(self) -> Option<u32> {
    if llapi::unlikely(self.is_zero()) {
      None
    } else {
      // SAFETY: We just ensured that `self` is not zero.
      Some(unsafe { logxx::ilog10::<N>(self) })
    }
  }

  #[doc = include_doc!(uint, "checked_signed_diff")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn checked_signed_diff(self, rhs: Self) -> Option<int<N>> {
    let out: int<N> = self.wrapping_sub(rhs).cast_signed();
    let cmp: bool = self.const_ge(&rhs) == out.is_negative();

    if llapi::unlikely(cmp) {
      None
    } else {
      Some(out)
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_checked_add, () => {
    assert_eq!(T::MIN.checked_add(T::P_1), Some(T::P_1));
    assert_eq!(T::MIN.checked_add(T::P_2), Some(T::P_2));
    assert_eq!(T::MIN.checked_add(T::MIN), Some(T::MIN));
    assert_eq!(T::MIN.checked_add(T::MAX), Some(T::MAX));

    assert_eq!(T::MAX.checked_add(T::P_1), None);
    assert_eq!(T::MAX.checked_add(T::P_2), None);
    assert_eq!(T::MAX.checked_add(T::MIN), Some(T::MAX));
    assert_eq!(T::MAX.checked_add(T::MAX), None);
  });

  test!(@uint, test_checked_add_signed, () => {
    assert_eq!(T::MIN.checked_add_signed(T::P_1.cast_signed()), Some(T::P_1));
    assert_eq!(T::MIN.checked_add_signed(T::P_2.cast_signed()), Some(T::P_2));
    assert_eq!(T::MIN.checked_add_signed(T::MIN.cast_signed()), Some(T::MIN));
    assert_eq!(T::MIN.checked_add_signed(T::MAX.cast_signed()), None);

    assert_eq!(T::MAX.checked_add_signed(T::P_1.cast_signed()), None);
    assert_eq!(T::MAX.checked_add_signed(T::P_2.cast_signed()), None);
    assert_eq!(T::MAX.checked_add_signed(T::MIN.cast_signed()), Some(T::MAX));
    assert_eq!(T::MAX.checked_add_signed(T::MAX.cast_signed()), Some(T::MAX - T::P_1));
  });

  test!(@uint, test_checked_sub, () => {
    assert_eq!(T::MIN.checked_sub(T::P_1), None);
    assert_eq!(T::MIN.checked_sub(T::P_2), None);
    assert_eq!(T::MIN.checked_sub(T::MIN), Some(T::MIN));
    assert_eq!(T::MIN.checked_sub(T::MAX), None);

    assert_eq!(T::MAX.checked_sub(T::P_1), Some(T::MAX - T::P_1));
    assert_eq!(T::MAX.checked_sub(T::P_2), Some(T::MAX - T::P_2));
    assert_eq!(T::MAX.checked_sub(T::MIN), Some(T::MAX));
    assert_eq!(T::MAX.checked_sub(T::MAX), Some(T::MIN));
  });

  test!(@uint, test_checked_sub_signed, () => {
    assert_eq!(T::MIN.checked_sub_signed(T::P_1.cast_signed()), None);
    assert_eq!(T::MIN.checked_sub_signed(T::P_2.cast_signed()), None);
    assert_eq!(T::MIN.checked_sub_signed(T::MIN.cast_signed()), Some(T::MIN));
    assert_eq!(T::MIN.checked_sub_signed(T::MAX.cast_signed()), Some(T::P_1));

    assert_eq!(T::MAX.checked_sub_signed(T::P_1.cast_signed()), Some(T::MAX - T::P_1));
    assert_eq!(T::MAX.checked_sub_signed(T::P_2.cast_signed()), Some(T::MAX - T::P_2));
    assert_eq!(T::MAX.checked_sub_signed(T::MIN.cast_signed()), Some(T::MAX));
    assert_eq!(T::MAX.checked_sub_signed(T::MAX.cast_signed()), None);
  });

  test!(@uint, test_checked_mul, () => {
    assert_eq!(T::MIN.checked_mul(T::P_1), Some(T::MIN));
    assert_eq!(T::MIN.checked_mul(T::P_2), Some(T::MIN));
    assert_eq!(T::MIN.checked_mul(T::MIN), Some(T::MIN));
    assert_eq!(T::MIN.checked_mul(T::MAX), Some(T::MIN));

    assert_eq!(T::MAX.checked_mul(T::P_1), Some(T::MAX));
    assert_eq!(T::MAX.checked_mul(T::P_2), None);
    assert_eq!(T::MAX.checked_mul(T::MIN), Some(T::MIN));
    assert_eq!(T::MAX.checked_mul(T::MAX), None);
  });

  test!(@uint, test_checked_div, () => {
    assert_eq!(T::P_1.checked_div(T::P_2), Some(T::MIN));
    assert_eq!(T::P_2.checked_div(T::P_2), Some(T::P_1));
    assert_eq!(T::MIN.checked_div(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_div(T::P_2), Some(T::MAX / T::P_2));

    assert_eq!(T::P_1.checked_div(T::MIN), None);
    assert_eq!(T::P_2.checked_div(T::MIN), None);
    assert_eq!(T::MIN.checked_div(T::MIN), None);
    assert_eq!(T::MAX.checked_div(T::MIN), None);
  });

  test!(@uint, test_checked_div_euclid, () => {
    assert_eq!(T::P_1.checked_div_euclid(T::P_2), Some(T::MIN));
    assert_eq!(T::P_2.checked_div_euclid(T::P_2), Some(T::P_1));
    assert_eq!(T::MIN.checked_div_euclid(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_div_euclid(T::P_2), Some(T::MAX / T::P_2));

    assert_eq!(T::P_1.checked_div_euclid(T::MIN), None);
    assert_eq!(T::P_2.checked_div_euclid(T::MIN), None);
    assert_eq!(T::MIN.checked_div_euclid(T::MIN), None);
    assert_eq!(T::MAX.checked_div_euclid(T::MIN), None);
  });

  test!(@uint, test_checked_div_exact, () => {
    assert_eq!(T::P_1.checked_div_exact(T::P_2), None);
    assert_eq!(T::P_2.checked_div_exact(T::P_2), Some(T::P_1));
    assert_eq!(T::MIN.checked_div_exact(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_div_exact(T::P_2), None);

    assert_eq!(T::P_1.checked_div_exact(T::MIN), None);
    assert_eq!(T::P_2.checked_div_exact(T::MIN), None);
    assert_eq!(T::MIN.checked_div_exact(T::MIN), None);
    assert_eq!(T::MAX.checked_div_exact(T::MIN), None);
  });

  test!(@uint, test_checked_rem, () => {
    assert_eq!(T::P_1.checked_rem(T::P_2), Some(T::P_1));
    assert_eq!(T::P_2.checked_rem(T::P_2), Some(T::MIN));
    assert_eq!(T::MIN.checked_rem(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_rem(T::P_2), Some(T::MAX % T::P_2));

    assert_eq!(T::P_1.checked_rem(T::MIN), None);
    assert_eq!(T::P_2.checked_rem(T::MIN), None);
    assert_eq!(T::MIN.checked_rem(T::MIN), None);
    assert_eq!(T::MAX.checked_rem(T::MIN), None);
  });

  test!(@uint, test_checked_rem_euclid, () => {
    assert_eq!(T::P_1.checked_rem_euclid(T::P_2), Some(T::P_1));
    assert_eq!(T::P_2.checked_rem_euclid(T::P_2), Some(T::MIN));
    assert_eq!(T::MIN.checked_rem_euclid(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_rem_euclid(T::P_2), Some(T::MAX % T::P_2));

    assert_eq!(T::P_1.checked_rem_euclid(T::MIN), None);
    assert_eq!(T::P_2.checked_rem_euclid(T::MIN), None);
    assert_eq!(T::MIN.checked_rem_euclid(T::MIN), None);
    assert_eq!(T::MAX.checked_rem_euclid(T::MIN), None);
  });

  test!(@uint, test_checked_shl, () => {
    assert_checked_shift!(checked_shl, T, T::P_1);
    assert_checked_shift!(checked_shl, T, T::P_8);
    assert_checked_shift!(checked_shl, T, T::P_17);
    assert_checked_shift!(checked_shl, T, T::MIN);
    assert_checked_shift!(checked_shl, T, T::MAX);
  });

  test!(@uint, test_checked_shr, () => {
    assert_checked_shift!(checked_shr, T, T::P_1);
    assert_checked_shift!(checked_shr, T, T::P_8);
    assert_checked_shift!(checked_shr, T, T::P_17);
    assert_checked_shift!(checked_shr, T, T::MIN);
    assert_checked_shift!(checked_shr, T, T::MAX);
  });

  test!(@uint, test_checked_neg, () => {
    assert_eq!(T::P_1.checked_neg(), None);
    assert_eq!(T::P_2.checked_neg(), None);
    assert_eq!(T::MIN.checked_neg(), Some(T::MIN));
    assert_eq!(T::MAX.checked_neg(), None);
  });

  test!(@uint, test_checked_pow, () => {
    assert_eq!(T::P_1.checked_pow(2), Some(T::P_1));
    assert_eq!(T::P_2.checked_pow(2), Some(T::P_2 + T::P_2));
    assert_eq!(T::MIN.checked_pow(2), Some(T::MIN));
    assert_eq!(T::MAX.checked_pow(2), None);
  });

  test!(@uint, test_checked_next_power_of_two, () => {
    assert_eq!(T::P_1.checked_next_power_of_two(), Some(T::P_1));
    assert_eq!(T::P_2.checked_next_power_of_two(), Some(T::P_2));
    assert_eq!(T::MIN.checked_next_power_of_two(), Some(T::P_1));
    assert_eq!(T::MAX.checked_next_power_of_two(), None);
  });

  test!(@uint, test_checked_next_multiple_of, () => {
    assert_eq!(T::P_1.checked_next_multiple_of(T::P_2), Some(T::P_2));
    assert_eq!(T::P_2.checked_next_multiple_of(T::P_2), Some(T::P_2));
    assert_eq!(T::MIN.checked_next_multiple_of(T::P_2), Some(T::MIN));
    assert_eq!(T::MAX.checked_next_multiple_of(T::P_2), None);

    assert_eq!(T::P_1.checked_next_multiple_of(T::MIN), None);
    assert_eq!(T::P_2.checked_next_multiple_of(T::MIN), None);
    assert_eq!(T::MIN.checked_next_multiple_of(T::MIN), None);
    assert_eq!(T::MAX.checked_next_multiple_of(T::MIN), None);
  });

  test!(#[no_miri] @uint, test_checked_ilog, () => {
    assert_eq!(T::P_1.checked_ilog(T::P_0), None);
    assert_eq!(T::P_1.checked_ilog(T::P_1), None);
    assert_eq!(T::P_0.checked_ilog(T::P_13), None);

    for value in T::P_1..=T::P_255 {
      assert_ilog!(checked_ilog(T::P_13), T, value);
    }

    for value in T::iter() {
      assert_ilog!(checked_ilog(T::P_13), T, value);
    }
  });

  test!(#[no_miri] @uint, test_checked_ilog2, () => {
    assert_eq!(T::P_0.checked_ilog2(), None);

    for value in T::P_1..=T::P_255 {
      assert_ilog!(checked_ilog2, T, value);
    }

    for value in T::iter() {
      assert_ilog!(checked_ilog2, T, value);
    }
  });

  test!(#[no_miri] @uint, test_checked_ilog10, () => {
    assert_eq!(T::P_0.checked_ilog10(), None);

    for value in T::P_1..=T::P_255 {
      assert_ilog!(checked_ilog10, T, value);
    }

    for value in T::iter() {
      assert_ilog!(checked_ilog10, T, value);
    }
  });

  test!(@uint, test_checked_signed_diff, () => {
    assert_eq!(T::MIN.checked_signed_diff(T::P_1), Some(T::N_1.cast_signed()));
    assert_eq!(T::MIN.checked_signed_diff(T::P_2), Some(T::N_2.cast_signed()));
    assert_eq!(T::MIN.checked_signed_diff(T::MIN), Some(T::P_0.cast_signed()));
    assert_eq!(T::MIN.checked_signed_diff(T::MAX), None);

    assert_eq!(T::MAX.checked_signed_diff(T::P_1), None);
    assert_eq!(T::MAX.checked_signed_diff(T::P_2), None);
    assert_eq!(T::MAX.checked_signed_diff(T::MIN), None);
    assert_eq!(T::MAX.checked_signed_diff(T::MAX), Some(T::P_0.cast_signed()));
  });
}
