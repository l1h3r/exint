use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "wrapping_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add(self, rhs: Self) -> Self {
    llapi::wrapping_add::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "wrapping_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_add_unsigned(self, rhs: uint<N>) -> Self {
    self.wrapping_add(rhs.cast_signed())
  }

  #[doc = include_doc!(int, "wrapping_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub(self, rhs: Self) -> Self {
    llapi::wrapping_sub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "wrapping_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_sub_unsigned(self, rhs: uint<N>) -> Self {
    self.wrapping_sub(rhs.cast_signed())
  }

  #[doc = include_doc!(int, "wrapping_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_mul(self, rhs: Self) -> Self {
    llapi::wrapping_mul::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "wrapping_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div(self, rhs: Self) -> Self {
    self.overflowing_div(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_div_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
    self.overflowing_div_euclid(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_rem")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem(self, rhs: Self) -> Self {
    self.overflowing_rem(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
    self.overflowing_rem_euclid(rhs).0
  }

  #[doc = include_doc!(int, "wrapping_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shl(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shl(Self::mask(rhs)) }
  }

  #[doc = include_doc!(int, "wrapping_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_shr(self, rhs: u32) -> Self {
    // SAFETY: We mask `rhs` by `Self::BITS` which ensures we stay in-bounds.
    unsafe { self.unchecked_shr(Self::mask(rhs)) }
  }

  #[doc = include_doc!(int, "wrapping_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_neg(self) -> Self {
    Self::ZERO.wrapping_sub(self)
  }

  #[doc = include_doc!(int, "wrapping_pow")]
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

  #[doc = include_doc!(int, "wrapping_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn wrapping_abs(self) -> Self {
    if self.is_negative() {
      self.wrapping_neg()
    } else {
      self
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_wrapping_add, () => {
    assert_eq!(T::MIN.wrapping_add(T::N_1), T::MAX);
    assert_eq!(T::MIN.wrapping_add(T::P_1), T::MIN + T::P_1);
    assert_eq!(T::MIN.wrapping_add(T::MIN), T::P_0);
    assert_eq!(T::MIN.wrapping_add(T::MAX), T::N_1);

    assert_eq!(T::MAX.wrapping_add(T::N_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_add(T::P_1), T::MIN);
    assert_eq!(T::MAX.wrapping_add(T::MIN), T::N_1);
    assert_eq!(T::MAX.wrapping_add(T::MAX), T::N_1 + T::N_1);
  });

  test!(@sint, test_wrapping_add_unsigned, () => {
    assert_eq!(T::MIN.wrapping_add_unsigned(T::N_1.cast_unsigned()), T::MAX);
    assert_eq!(T::MIN.wrapping_add_unsigned(T::P_1.cast_unsigned()), T::MIN + T::P_1);
    assert_eq!(T::MIN.wrapping_add_unsigned(T::MIN.cast_unsigned()), T::P_0);
    assert_eq!(T::MIN.wrapping_add_unsigned(T::MAX.cast_unsigned()), T::N_1);

    assert_eq!(T::MAX.wrapping_add_unsigned(T::N_1.cast_unsigned()), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_add_unsigned(T::P_1.cast_unsigned()), T::MIN);
    assert_eq!(T::MAX.wrapping_add_unsigned(T::MIN.cast_unsigned()), T::N_1);
    assert_eq!(T::MAX.wrapping_add_unsigned(T::MAX.cast_unsigned()), T::N_1 + T::N_1);
  });

  test!(@sint, test_wrapping_sub, () => {
    assert_eq!(T::MIN.wrapping_sub(T::N_1), T::MIN + T::P_1);
    assert_eq!(T::MIN.wrapping_sub(T::P_1), T::MAX);
    assert_eq!(T::MIN.wrapping_sub(T::MIN), T::P_0);
    assert_eq!(T::MIN.wrapping_sub(T::MAX), T::N_1.abs());

    assert_eq!(T::MAX.wrapping_sub(T::N_1), T::MIN);
    assert_eq!(T::MAX.wrapping_sub(T::P_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_sub(T::MIN), T::N_1);
    assert_eq!(T::MAX.wrapping_sub(T::MAX), T::P_0);
  });

  test!(@sint, test_wrapping_sub_unsigned, () => {
    assert_eq!(T::MIN.wrapping_sub_unsigned(T::N_1.cast_unsigned()), T::MIN + T::P_1);
    assert_eq!(T::MIN.wrapping_sub_unsigned(T::P_1.cast_unsigned()), T::MAX);
    assert_eq!(T::MIN.wrapping_sub_unsigned(T::MIN.cast_unsigned()), T::P_0);
    assert_eq!(T::MIN.wrapping_sub_unsigned(T::MAX.cast_unsigned()), T::N_1.abs());

    assert_eq!(T::MAX.wrapping_sub_unsigned(T::N_1.cast_unsigned()), T::MIN);
    assert_eq!(T::MAX.wrapping_sub_unsigned(T::P_1.cast_unsigned()), T::MAX - T::P_1);
    assert_eq!(T::MAX.wrapping_sub_unsigned(T::MIN.cast_unsigned()), T::N_1);
    assert_eq!(T::MAX.wrapping_sub_unsigned(T::MAX.cast_unsigned()), T::P_0);
  });

  test!(@sint, test_wrapping_mul, () => {
    assert_eq!(T::MIN.wrapping_mul(T::N_1), T::MIN);
    assert_eq!(T::MIN.wrapping_mul(T::P_1), T::MIN);
    assert_eq!(T::MIN.wrapping_mul(T::MIN), T::P_0);
    assert_eq!(T::MIN.wrapping_mul(T::MAX), T::MIN);

    assert_eq!(T::MAX.wrapping_mul(T::N_1), -T::MAX);
    assert_eq!(T::MAX.wrapping_mul(T::P_1), T::MAX);
    assert_eq!(T::MAX.wrapping_mul(T::MIN), T::MIN);
    assert_eq!(T::MAX.wrapping_mul(T::MAX), T::P_1);
  });

  test!(@sint, test_wrapping_div, () => {
    // Forwards to overflowing_div
  });

  test!(@sint, test_wrapping_div_euclid, () => {
    // Forwards to overflowing_div_euclid
  });

  test!(@sint, test_wrapping_rem, () => {
    // Forwards to overflowing_rem
  });

  test!(@sint, test_wrapping_rem_euclid, () => {
    // Forwards to overflowing_rem_euclid
  });

  test!(@sint, test_wrapping_shl, () => {
    assert_wrapping_shift!(wrapping_shl, T, T::N_1);
    assert_wrapping_shift!(wrapping_shl, T, T::P_1);
    assert_wrapping_shift!(wrapping_shl, T, T::P_8);
    assert_wrapping_shift!(wrapping_shl, T, T::P_17);
    assert_wrapping_shift!(wrapping_shl, T, T::MIN);
    assert_wrapping_shift!(wrapping_shl, T, T::MAX);
  });

  test!(@sint, test_wrapping_shr, () => {
    assert_wrapping_shift!(wrapping_shr, T, T::N_1);
    assert_wrapping_shift!(wrapping_shr, T, T::P_1);
    assert_wrapping_shift!(wrapping_shr, T, T::P_8);
    assert_wrapping_shift!(wrapping_shr, T, T::P_17);
    assert_wrapping_shift!(wrapping_shr, T, T::MIN);
    assert_wrapping_shift!(wrapping_shr, T, T::MAX);
  });

  test!(@sint, test_wrapping_neg, () => {
    assert_eq!(T::N_1.wrapping_neg(), T::P_1);
    assert_eq!(T::P_0.wrapping_neg(), T::P_0);
    assert_eq!(T::P_1.wrapping_neg(), T::N_1);
    assert_eq!(T::MIN.wrapping_neg(), T::MIN);
    assert_eq!(T::MAX.wrapping_neg(), T::MIN + T::P_1);
  });

  test!(@sint, test_wrapping_pow, () => {
    assert_eq!(T::N_1.wrapping_pow(2), T::P_1);
    assert_eq!(T::P_0.wrapping_pow(2), T::P_0);
    assert_eq!(T::P_1.wrapping_pow(2), T::P_1);
    assert_eq!(T::MIN.wrapping_pow(2), T::P_0);
    assert_eq!(T::MAX.wrapping_pow(2), T::P_1);
  });

  test!(@sint, test_wrapping_abs, () => {
    assert_eq!(T::N_1.wrapping_abs(), T::P_1);
    assert_eq!(T::P_0.wrapping_abs(), T::P_0);
    assert_eq!(T::P_1.wrapping_abs(), T::P_1);
    assert_eq!(T::MIN.wrapping_abs(), T::MIN);
    assert_eq!(T::MAX.wrapping_abs(), T::MAX);
  });
}
