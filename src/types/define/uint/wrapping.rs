use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "wrapping_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add(self, rhs: Self) -> Self {
    llapi::wrapping_add::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "wrapping_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add_signed(self, rhs: int<N>) -> Self {
    self.wrapping_add(rhs.cast_unsigned())
  }

  #[doc = include_doc!(uint, "wrapping_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub(self, rhs: Self) -> Self {
    llapi::wrapping_sub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "wrapping_sub_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub_signed(self, rhs: int<N>) -> Self {
    self.wrapping_sub(rhs.cast_unsigned())
  }

  #[doc = include_doc!(uint, "wrapping_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_mul(self, rhs: Self) -> Self {
    llapi::wrapping_mul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "wrapping_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "wrapping_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shl(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shl(Self::mask(rhs)) }
  }

  #[doc = include_doc!(uint, "wrapping_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shr(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shr(Self::mask(rhs)) }
  }

  #[doc = include_doc!(uint, "wrapping_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_neg(self) -> Self {
    Self::ZERO.wrapping_sub(self)
  }

  #[doc = include_doc!(uint, "wrapping_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_pow(self, mut exp: u32) -> Self {
    if exp == 0 {
      return Self::ONE;
    }

    let mut base: Self = self;
    let mut acc: Self = Self::ONE;

    #[cfg(feature = "core_intrinsics")]
    if ::core::intrinsics::is_val_statically_known(exp) {
      while exp > 1 {
        if (exp & 1) == 1 {
          acc = acc.wrapping_mul(base);
        }

        exp /= 2;
        base = base.wrapping_mul(base);
      }

      return acc.wrapping_mul(base);
    }

    loop {
      if (exp & 1) == 1 {
        acc = acc.wrapping_mul(base);

        if exp == 1 {
          return acc;
        }
      }

      exp /= 2;
      base = base.wrapping_mul(base);
    }
  }

  #[doc = include_doc!(uint, "wrapping_next_power_of_two")]
  #[cfg(feature = "wrapping_next_power_of_two")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_next_power_of_two(self) -> Self {
    self
      .one_less_than_next_power_of_two()
      .wrapping_add(Self::ONE)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_wrapping_add, () => {
    assert_eq!(T::MIN.wrapping_add(T::P_1), T::P_1);
    assert_eq!(T::MIN.wrapping_add(T::P_2), T::P_2);
    assert_eq!(T::MIN.wrapping_add(T::MIN), T::MIN);
    assert_eq!(T::MIN.wrapping_add(T::MAX), T::MAX);

    assert_eq!(T::MAX.wrapping_add(T::P_1), T::MIN);
    assert_eq!(T::MAX.wrapping_add(T::P_2), T::P_1);
    assert_eq!(T::MAX.wrapping_add(T::MIN), T::MAX);
    assert_eq!(T::MAX.wrapping_add(T::MAX), T::MAX - T::P_1);
  });

  test!(@uint, test_wrapping_add_signed, () => {
    assert_eq!(T::MIN.wrapping_add_signed(T::P_1.cast_signed()), T::P_1);
    assert_eq!(T::MIN.wrapping_add_signed(T::P_2.cast_signed()), T::P_2);
    assert_eq!(T::MIN.wrapping_add_signed(T::MIN.cast_signed()), T::MIN);
    assert_eq!(T::MIN.wrapping_add_signed(T::MAX.cast_signed()), T::MAX);

    assert_eq!(T::MAX.wrapping_add_signed(T::P_1.cast_signed()), T::MIN);
    assert_eq!(T::MAX.wrapping_add_signed(T::P_2.cast_signed()), T::P_1);
    assert_eq!(T::MAX.wrapping_add_signed(T::MIN.cast_signed()), T::MAX);
    assert_eq!(T::MAX.wrapping_add_signed(T::MAX.cast_signed()), T::MAX - T::P_1);
  });

  test!(@uint, test_wrapping_sub, () => {
    assert_eq!(T::MIN.wrapping_sub(T::P_1), T::MAX);
    assert_eq!(T::MIN.wrapping_sub(T::P_2), T::MAX - T::P_1);
    assert_eq!(T::MIN.wrapping_sub(T::MIN), T::MIN);
    assert_eq!(T::MIN.wrapping_sub(T::MAX), T::P_1);

    assert_eq!(T::MAX.wrapping_sub(T::P_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_sub(T::P_2), T::MAX - T::P_2);
    assert_eq!(T::MAX.wrapping_sub(T::MIN), T::MAX);
    assert_eq!(T::MAX.wrapping_sub(T::MAX), T::MIN);
  });

  test!(@uint, test_wrapping_sub_signed, () => {
    assert_eq!(T::MIN.wrapping_sub_signed(T::P_1.cast_signed()), T::MAX);
    assert_eq!(T::MIN.wrapping_sub_signed(T::P_2.cast_signed()), T::MAX - T::P_1);
    assert_eq!(T::MIN.wrapping_sub_signed(T::MIN.cast_signed()), T::MIN);
    assert_eq!(T::MIN.wrapping_sub_signed(T::MAX.cast_signed()), T::P_1);

    assert_eq!(T::MAX.wrapping_sub_signed(T::P_1.cast_signed()), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_sub_signed(T::P_2.cast_signed()), T::MAX - T::P_2);
    assert_eq!(T::MAX.wrapping_sub_signed(T::MIN.cast_signed()), T::MAX);
    assert_eq!(T::MAX.wrapping_sub_signed(T::MAX.cast_signed()), T::MIN);
  });

  test!(@uint, test_wrapping_mul, () => {
    assert_eq!(T::MIN.wrapping_mul(T::P_1), T::MIN);
    assert_eq!(T::MIN.wrapping_mul(T::P_2), T::MIN);
    assert_eq!(T::MIN.wrapping_mul(T::MIN), T::MIN);
    assert_eq!(T::MIN.wrapping_mul(T::MAX), T::MIN);

    assert_eq!(T::MAX.wrapping_mul(T::P_1), T::MAX);
    assert_eq!(T::MAX.wrapping_mul(T::P_2), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_mul(T::MIN), T::MIN);
    assert_eq!(T::MAX.wrapping_mul(T::MAX), T::P_1);
  });

  test!(@uint, test_wrapping_div, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_wrapping_div_euclid, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_wrapping_rem, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_wrapping_rem_euclid, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_wrapping_shl, () => {
    assert_wrapping_shift!(wrapping_shl, T, T::P_1);
    assert_wrapping_shift!(wrapping_shl, T, T::P_8);
    assert_wrapping_shift!(wrapping_shl, T, T::P_17);
    assert_wrapping_shift!(wrapping_shl, T, T::MIN);
    assert_wrapping_shift!(wrapping_shl, T, T::MAX);
  });

  test!(@uint, test_wrapping_shr, () => {
    assert_wrapping_shift!(wrapping_shr, T, T::P_1);
    assert_wrapping_shift!(wrapping_shr, T, T::P_8);
    assert_wrapping_shift!(wrapping_shr, T, T::P_17);
    assert_wrapping_shift!(wrapping_shr, T, T::MIN);
    assert_wrapping_shift!(wrapping_shr, T, T::MAX);
  });

  test!(@uint, test_wrapping_neg, () => {
    assert_eq!(T::P_1.wrapping_neg(), T::MAX);
    assert_eq!(T::P_2.wrapping_neg(), T::MAX - T::P_1);
    assert_eq!(T::MIN.wrapping_neg(), T::MIN);
    assert_eq!(T::MAX.wrapping_neg(), T::P_1);
  });

  test!(@uint, test_wrapping_pow, () => {
    assert_eq!(T::P_1.wrapping_pow(2), T::P_1);
    assert_eq!(T::P_2.wrapping_pow(2), T::P_2 + T::P_2);
    assert_eq!(T::MIN.wrapping_pow(2), T::MIN);
    assert_eq!(T::MAX.wrapping_pow(2), T::P_1);
  });

  test!(@uint, test_wrapping_next_power_of_two, () => {
    assert_eq!(T::P_1.wrapping_next_power_of_two(), T::P_1);
    assert_eq!(T::P_2.wrapping_next_power_of_two(), T::P_2);
    assert_eq!(T::MIN.wrapping_next_power_of_two(), T::P_1);
    assert_eq!(T::MAX.wrapping_next_power_of_two(), T::MIN);
  });
}
