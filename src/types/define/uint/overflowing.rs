use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "overflowing_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_uadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "overflowing_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_add_signed(self, rhs: int<N>) -> (Self, bool) {
    let out: (Self, bool) = self.overflowing_add(rhs.cast_unsigned());
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(uint, "overflowing_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_usub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "overflowing_sub_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_sub_signed(self, rhs: int<N>) -> (Self, bool) {
    let out: (Self, bool) = self.overflowing_sub(rhs.cast_unsigned());
    let cmp: bool = out.1 ^ rhs.is_negative();

    (out.0, cmp)
  }

  #[doc = include_doc!(uint, "overflowing_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
    llapi::overflowing_umul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "overflowing_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
    (self.const_div(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
    (self.const_div(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
    (self.const_rem(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
    (self.const_rem(rhs), false)
  }

  #[doc = include_doc!(uint, "overflowing_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shl(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(uint, "overflowing_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
    (self.wrapping_shr(rhs), rhs >= Self::BITS)
  }

  #[doc = include_doc!(uint, "overflowing_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn overflowing_neg(self) -> (Self, bool) {
    (self.const_not().wrapping_add(Self::ONE), !self.is_zero())
  }

  #[doc = include_doc!(uint, "overflowing_pow")]
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
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_overflowing_add, () => {
    assert_eq!(T::MIN.overflowing_add(T::P_1), (T::P_1, false));
    assert_eq!(T::MIN.overflowing_add(T::P_2), (T::P_2, false));
    assert_eq!(T::MIN.overflowing_add(T::MIN), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_add(T::MAX), (T::MAX, false));

    assert_eq!(T::MAX.overflowing_add(T::P_1), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_add(T::P_2), (T::P_1, true));
    assert_eq!(T::MAX.overflowing_add(T::MIN), (T::MAX, false));
    assert_eq!(T::MAX.overflowing_add(T::MAX), (T::MAX - T::P_1, true));
  });

  test!(@uint, test_overflowing_add_signed, () => {
    assert_eq!(T::MIN.overflowing_add_signed(T::P_1.cast_signed()), (T::P_1, false));
    assert_eq!(T::MIN.overflowing_add_signed(T::P_2.cast_signed()), (T::P_2, false));
    assert_eq!(T::MIN.overflowing_add_signed(T::MIN.cast_signed()), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_add_signed(T::MAX.cast_signed()), (T::MAX, true));

    assert_eq!(T::MAX.overflowing_add_signed(T::P_1.cast_signed()), (T::MIN, true));
    assert_eq!(T::MAX.overflowing_add_signed(T::P_2.cast_signed()), (T::P_1, true));
    assert_eq!(T::MAX.overflowing_add_signed(T::MIN.cast_signed()), (T::MAX, false));
    assert_eq!(T::MAX.overflowing_add_signed(T::MAX.cast_signed()), (T::MAX - T::P_1, false));
  });

  test!(@uint, test_overflowing_sub, () => {
    assert_eq!(T::MIN.overflowing_sub(T::P_1), (T::MAX, true));
    assert_eq!(T::MIN.overflowing_sub(T::P_2), (T::MAX - T::P_1, true));
    assert_eq!(T::MIN.overflowing_sub(T::MIN), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_sub(T::MAX), (T::P_1, true));

    assert_eq!(T::MAX.overflowing_sub(T::P_1), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.overflowing_sub(T::P_2), (T::MAX - T::P_2, false));
    assert_eq!(T::MAX.overflowing_sub(T::MIN), (T::MAX, false));
    assert_eq!(T::MAX.overflowing_sub(T::MAX), (T::MIN, false));
  });

  test!(@uint, test_overflowing_sub_signed, () => {
    assert_eq!(T::MIN.overflowing_sub_signed(T::P_1.cast_signed()), (T::MAX, true));
    assert_eq!(T::MIN.overflowing_sub_signed(T::P_2.cast_signed()), (T::MAX - T::P_1, true));
    assert_eq!(T::MIN.overflowing_sub_signed(T::MIN.cast_signed()), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_sub_signed(T::MAX.cast_signed()), (T::P_1, false));

    assert_eq!(T::MAX.overflowing_sub_signed(T::P_1.cast_signed()), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.overflowing_sub_signed(T::P_2.cast_signed()), (T::MAX - T::P_2, false));
    assert_eq!(T::MAX.overflowing_sub_signed(T::MIN.cast_signed()), (T::MAX, false));
    assert_eq!(T::MAX.overflowing_sub_signed(T::MAX.cast_signed()), (T::MIN, true));
  });

  test!(@uint, test_overflowing_mul, () => {
    assert_eq!(T::MIN.overflowing_mul(T::P_1), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_mul(T::P_2), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_mul(T::MIN), (T::MIN, false));
    assert_eq!(T::MIN.overflowing_mul(T::MAX), (T::MIN, false));

    assert_eq!(T::MAX.overflowing_mul(T::P_1), (T::MAX, false));
    assert_eq!(T::MAX.overflowing_mul(T::P_2), (T::MAX - T::P_1, true));
    assert_eq!(T::MAX.overflowing_mul(T::MIN), (T::MIN, false));
    assert_eq!(T::MAX.overflowing_mul(T::MAX), (T::P_1, true));
  });

  test!(@uint, test_overflowing_div, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_overflowing_div_euclid, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_overflowing_rem, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_overflowing_rem_euclid, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_overflowing_shl, () => {
    assert_overflowing_shift!(overflowing_shl, T, T::P_1);
    assert_overflowing_shift!(overflowing_shl, T, T::P_8);
    assert_overflowing_shift!(overflowing_shl, T, T::P_17);
    assert_overflowing_shift!(overflowing_shl, T, T::MIN);
    assert_overflowing_shift!(overflowing_shl, T, T::MAX);
  });

  test!(@uint, test_overflowing_shr, () => {
    assert_overflowing_shift!(overflowing_shr, T, T::P_1);
    assert_overflowing_shift!(overflowing_shr, T, T::P_8);
    assert_overflowing_shift!(overflowing_shr, T, T::P_17);
    assert_overflowing_shift!(overflowing_shr, T, T::MIN);
    assert_overflowing_shift!(overflowing_shr, T, T::MAX);
  });

  test!(@uint, test_overflowing_neg, () => {
    assert_eq!(T::P_1.overflowing_neg(), (T::MAX, true));
    assert_eq!(T::P_2.overflowing_neg(), (T::MAX - T::P_1, true));
    assert_eq!(T::MIN.overflowing_neg(), (T::MIN, false));
    assert_eq!(T::MAX.overflowing_neg(), (T::P_1, true));
  });

  test!(@uint, test_overflowing_pow, () => {
    assert_eq!(T::P_1.overflowing_pow(2), (T::P_1, false));
    assert_eq!(T::P_2.overflowing_pow(2), (T::P_2 + T::P_2, false));
    assert_eq!(T::MIN.overflowing_pow(2), (T::MIN, false));
    assert_eq!(T::MAX.overflowing_pow(2), (T::P_1, true));
  });
}
