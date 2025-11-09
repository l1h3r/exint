use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "overflowing_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_sadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "overflowing_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add_unsigned(self, rhs: uint<N>) -> (Self, bool) {
    let rhs: Self = rhs.cast_signed();
    let out: (Self, bool) = self.overflowing_add(rhs);
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(int, "overflowing_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_ssub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "overflowing_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub_unsigned(self, rhs: uint<N>) -> (Self, bool) {
    let rhs: Self = rhs.cast_signed();
    let out: (Self, bool) = self.overflowing_sub(rhs);
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(int, "overflowing_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_smul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "overflowing_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE)) {
      (self, true)
    } else {
      (self.const_div(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(self.const_eq(&Self::MIN) & rhs.const_eq(&Self::NEG_ONE)) {
      (self, true)
    } else {
      (self.div_euclid(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(rhs.const_eq(&Self::NEG_ONE)) {
      (Self::ZERO, self.const_eq(&Self::MIN))
    } else {
      (self.const_rem(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
    if llapi::unlikely(rhs.const_eq(&Self::NEG_ONE)) {
      (Self::ZERO, self.const_eq(&Self::MIN))
    } else {
      (self.rem_euclid(rhs), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shl(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(int, "overflowing_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shr(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(int, "overflowing_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_neg(self) -> (Self, bool) {
    if llapi::unlikely(self.const_eq(&Self::MIN)) {
      (Self::MIN, true)
    } else {
      (self.wrapping_neg(), false)
    }
  }

  #[doc = include_doc!(int, "overflowing_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
    if exp == 0 {
      return (Self::ONE, false);
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;
    let mut overflow: bool = false;
    let mut result: (Self, bool);

    loop {
      if (exp & 1) == 1 {
        result = acc.overflowing_mul(base);

        if exp == 1 {
          result.1 |= overflow;
          return result;
        }

        acc = result.0;
        overflow |= result.1;
      }

      exp /= 2;
      result = base.overflowing_mul(base);
      base = result.0;
      overflow |= result.1;
    }
  }

  #[doc = include_doc!(int, "overflowing_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_abs(self) -> (Self, bool) {
    (self.wrapping_abs(), self.const_eq(&Self::MIN))
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_overflowing_add, () => {
    assert_eq!(T::MIN.overflowing_add(T::N_1), (T::MAX, true));
    assert_eq!(T::MIN.overflowing_add(T::P_1), (T::MIN + T::P_1, false));
    assert_eq!(T::MIN.overflowing_add(T::MIN), (T::P_0, true));
    assert_eq!(T::MIN.overflowing_add(T::MAX), (T::N_1, false));

    assert_eq!(T::MAX.overflowing_add(T::N_1), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.overflowing_add(T::P_1), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_add(T::MIN), (T::N_1, false));
    assert_eq!(T::MAX.overflowing_add(T::MAX), (T::N_1 + T::N_1, true));
  });

  test!(@sint, test_overflowing_add_unsigned, () => {
    assert_eq!(T::MIN.overflowing_add_unsigned(T::N_1.cast_unsigned()), (T::MAX, false));
    assert_eq!(T::MIN.overflowing_add_unsigned(T::P_1.cast_unsigned()), (T::MIN + T::P_1, false));
    assert_eq!(T::MIN.overflowing_add_unsigned(T::MIN.cast_unsigned()), (T::P_0, false));
    assert_eq!(T::MIN.overflowing_add_unsigned(T::MAX.cast_unsigned()), (T::N_1, false));

    assert_eq!(T::MAX.overflowing_add_unsigned(T::N_1.cast_unsigned()), (T::MAX - T::P_1, true));
    assert_eq!(T::MAX.overflowing_add_unsigned(T::P_1.cast_unsigned()), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_add_unsigned(T::MIN.cast_unsigned()), (T::N_1, true));
    assert_eq!(T::MAX.overflowing_add_unsigned(T::MAX.cast_unsigned()), (T::N_1 + T::N_1, true));
  });

  test!(@sint, test_overflowing_sub, () => {
    assert_eq!(T::MIN.overflowing_sub(T::N_1), (T::MIN + T::P_1, false));
    assert_eq!(T::MIN.overflowing_sub(T::P_1), (T::MAX, true));
    assert_eq!(T::MIN.overflowing_sub(T::MIN), (T::P_0, false));
    assert_eq!(T::MIN.overflowing_sub(T::MAX), (T::N_1.abs(), true));

    assert_eq!(T::MAX.overflowing_sub(T::N_1), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_sub(T::P_1), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.overflowing_sub(T::MIN), (T::N_1, true));
    assert_eq!(T::MAX.overflowing_sub(T::MAX), (T::P_0, false));
  });

  test!(@sint, test_overflowing_sub_unsigned, () => {
    assert_eq!(T::MIN.overflowing_sub_unsigned(T::N_1.cast_unsigned()), (T::MIN + T::P_1, true));
    assert_eq!(T::MIN.overflowing_sub_unsigned(T::P_1.cast_unsigned()), (T::MAX, true));
    assert_eq!(T::MIN.overflowing_sub_unsigned(T::MIN.cast_unsigned()), (T::P_0, true));
    assert_eq!(T::MIN.overflowing_sub_unsigned(T::MAX.cast_unsigned()), (T::N_1.abs(), true));

    assert_eq!(T::MAX.overflowing_sub_unsigned(T::N_1.cast_unsigned()), (T::MIN, false));
    assert_eq!(T::MAX.overflowing_sub_unsigned(T::P_1.cast_unsigned()), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.overflowing_sub_unsigned(T::MIN.cast_unsigned()), (T::N_1, false));
    assert_eq!(T::MAX.overflowing_sub_unsigned(T::MAX.cast_unsigned()), (T::P_0, false));
  });

  test!(@sint, test_overflowing_mul, () => {
    assert_eq!(T::MIN.overflowing_mul(T::N_1), (T::MIN, true));
    assert_eq!(T::MIN.overflowing_mul(T::P_1), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_mul(T::MIN), (T::P_0, true));
    assert_eq!(T::MIN.overflowing_mul(T::MAX), (T::MIN, true));

    assert_eq!(T::MAX.overflowing_mul(T::N_1), (-T::MAX, false));
    assert_eq!(T::MAX.overflowing_mul(T::P_1), (T::MAX, false));
    assert_eq!(T::MAX.overflowing_mul(T::MIN), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_mul(T::MAX), (T::P_1, true));
  });

  test!(@sint, test_overflowing_div, () => {
    assert_eq!(T::N_1.overflowing_div(T::N_1), (T::P_1, false));
    assert_eq!(T::P_0.overflowing_div(T::N_1), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_div(T::N_1), (T::N_1, false));
    assert_eq!(T::MIN.overflowing_div(T::N_1), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_div(T::N_1), (T::MIN + T::P_1, false));

    assert_panic!(T::N_1.overflowing_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.overflowing_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.overflowing_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.overflowing_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.overflowing_div(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_overflowing_div_euclid, () => {
    assert_eq!(T::N_1.overflowing_div_euclid(T::N_1), (T::P_1, false));
    assert_eq!(T::P_0.overflowing_div_euclid(T::N_1), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_div_euclid(T::N_1), (T::N_1, false));
    assert_eq!(T::MIN.overflowing_div_euclid(T::N_1), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_div_euclid(T::N_1), (T::MIN + T::P_1, false));

    assert_panic!(T::N_1.overflowing_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.overflowing_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.overflowing_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.overflowing_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.overflowing_div_euclid(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_overflowing_rem, () => {
    assert_eq!(T::N_1.overflowing_rem(T::N_1), (T::P_0, false));
    assert_eq!(T::P_0.overflowing_rem(T::N_1), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_rem(T::N_1), (T::P_0, false));
    assert_eq!(T::MIN.overflowing_rem(T::N_1), (T::P_0, true));
    assert_eq!(T::MAX.overflowing_rem(T::N_1), (T::P_0, false));

    assert_panic!(T::N_1.overflowing_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_0.overflowing_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_1.overflowing_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::MIN.overflowing_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::MAX.overflowing_rem(T::P_0), message = REM_ZERO);
  });

  test!(@sint, test_overflowing_rem_euclid, () => {
    assert_eq!(T::N_1.overflowing_rem_euclid(T::N_1), (T::P_0, false));
    assert_eq!(T::P_0.overflowing_rem_euclid(T::N_1), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_rem_euclid(T::N_1), (T::P_0, false));
    assert_eq!(T::MIN.overflowing_rem_euclid(T::N_1), (T::P_0, true));
    assert_eq!(T::MAX.overflowing_rem_euclid(T::N_1), (T::P_0, false));

    assert_panic!(T::N_1.overflowing_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_0.overflowing_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_1.overflowing_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::MIN.overflowing_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::MAX.overflowing_rem_euclid(T::P_0), message = REM_ZERO);
  });

  test!(@sint, test_overflowing_shl, () => {
    assert_overflowing_shift!(overflowing_shl, T, T::N_1);
    assert_overflowing_shift!(overflowing_shl, T, T::P_1);
    assert_overflowing_shift!(overflowing_shl, T, T::P_8);
    assert_overflowing_shift!(overflowing_shl, T, T::P_17);
    assert_overflowing_shift!(overflowing_shl, T, T::MIN);
    assert_overflowing_shift!(overflowing_shl, T, T::MAX);
  });

  test!(@sint, test_overflowing_shr, () => {
    assert_overflowing_shift!(overflowing_shr, T, T::N_1);
    assert_overflowing_shift!(overflowing_shr, T, T::P_1);
    assert_overflowing_shift!(overflowing_shr, T, T::P_8);
    assert_overflowing_shift!(overflowing_shr, T, T::P_17);
    assert_overflowing_shift!(overflowing_shr, T, T::MIN);
    assert_overflowing_shift!(overflowing_shr, T, T::MAX);
  });

  test!(@sint, test_overflowing_neg, () => {
    assert_eq!(T::N_1.overflowing_neg(), (T::P_1, false));
    assert_eq!(T::P_0.overflowing_neg(), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_neg(), (T::N_1, false));
    assert_eq!(T::MIN.overflowing_neg(), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_neg(), (T::MIN + T::P_1, false));
  });

  test!(@sint, test_overflowing_pow, () => {
    assert_eq!(T::N_1.overflowing_pow(2), (T::P_1, false));
    assert_eq!(T::P_0.overflowing_pow(2), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_pow(2), (T::P_1, false));
    assert_eq!(T::MIN.overflowing_pow(2), (T::P_0, true));
    assert_eq!(T::MAX.overflowing_pow(2), (T::P_1, true));
  });

  test!(@sint, test_overflowing_abs, () => {
    assert_eq!(T::N_1.overflowing_abs(), (T::P_1, false));
    assert_eq!(T::P_0.overflowing_abs(), (T::P_0, false));
    assert_eq!(T::P_1.overflowing_abs(), (T::P_1, false));
    assert_eq!(T::MIN.overflowing_abs(), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_abs(), (T::MAX, false));
  });
}
