use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "saturating_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add(self, rhs: Self) -> Self {
    llapi::saturating_sadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "saturating_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_add_unsigned(rhs) {
      Some(result) => result,
      None => Self::MAX,
    }
  }

  #[doc = include_doc!(int, "saturating_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub(self, rhs: Self) -> Self {
    llapi::saturating_ssub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(int, "saturating_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_sub_unsigned(rhs) {
      Some(result) => result,
      None => Self::MIN,
    }
  }

  #[doc = include_doc!(int, "saturating_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None if self.is_negative() == rhs.is_negative() => Self::MAX,
      None => Self::MIN,
    }
  }

  #[doc = include_doc!(int, "saturating_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_div(self, rhs: Self) -> Self {
    match self.overflowing_div(rhs) {
      (result, false) => result,
      (_result, true) => Self::MAX,
    }
  }

  #[doc = include_doc!(int, "saturating_neg")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_neg(self) -> Self {
    llapi::saturating_ssub::<Self, N>(Self::ZERO, self)
  }

  #[doc = include_doc!(int, "saturating_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None if self.is_negative() && exp % 2 == 1 => Self::MIN,
      None => Self::MAX,
    }
  }

  #[doc = include_doc!(int, "saturating_abs")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_abs(self) -> Self {
    if self.is_negative() {
      self.saturating_neg()
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

  test!(@sint, test_saturating_add, () => {
    assert_eq!(T::MIN.saturating_add(T::N_1), T::MIN);
    assert_eq!(T::MIN.saturating_add(T::P_1), T::MIN + T::P_1);
    assert_eq!(T::MIN.saturating_add(T::MIN), T::MIN);
    assert_eq!(T::MIN.saturating_add(T::MAX), T::N_1);

    assert_eq!(T::MAX.saturating_add(T::N_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.saturating_add(T::P_1), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::MIN), T::N_1);
    assert_eq!(T::MAX.saturating_add(T::MAX), T::MAX);
  });

  test!(@sint, test_saturating_add_unsigned, () => {
    assert_eq!(T::MIN.saturating_add_unsigned(T::N_1.cast_unsigned()), T::MAX);
    assert_eq!(T::MIN.saturating_add_unsigned(T::P_1.cast_unsigned()), T::MIN + T::P_1);
    assert_eq!(T::MIN.saturating_add_unsigned(T::MIN.cast_unsigned()), T::P_0);
    assert_eq!(T::MIN.saturating_add_unsigned(T::MAX.cast_unsigned()), T::N_1);

    assert_eq!(T::MAX.saturating_add_unsigned(T::N_1.cast_unsigned()), T::MAX);
    assert_eq!(T::MAX.saturating_add_unsigned(T::P_1.cast_unsigned()), T::MAX);
    assert_eq!(T::MAX.saturating_add_unsigned(T::MIN.cast_unsigned()), T::MAX);
    assert_eq!(T::MAX.saturating_add_unsigned(T::MAX.cast_unsigned()), T::MAX);
  });

  test!(@sint, test_saturating_sub, () => {
    assert_eq!(T::MIN.saturating_sub(T::N_1), T::MIN + T::P_1);
    assert_eq!(T::MIN.saturating_sub(T::P_1), T::MIN);
    assert_eq!(T::MIN.saturating_sub(T::MIN), T::P_0);
    assert_eq!(T::MIN.saturating_sub(T::MAX), T::MIN);

    assert_eq!(T::MAX.saturating_sub(T::N_1), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::P_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.saturating_sub(T::MIN), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::MAX), T::P_0);
  });

  test!(@sint, test_saturating_sub_unsigned, () => {
    assert_eq!(T::MIN.saturating_sub_unsigned(T::N_1.cast_unsigned()), T::MIN);
    assert_eq!(T::MIN.saturating_sub_unsigned(T::P_1.cast_unsigned()), T::MIN);
    assert_eq!(T::MIN.saturating_sub_unsigned(T::MIN.cast_unsigned()), T::MIN);
    assert_eq!(T::MIN.saturating_sub_unsigned(T::MAX.cast_unsigned()), T::MIN);

    assert_eq!(T::MAX.saturating_sub_unsigned(T::N_1.cast_unsigned()), T::MIN);
    assert_eq!(T::MAX.saturating_sub_unsigned(T::P_1.cast_unsigned()), T::MAX - T::P_1);
    assert_eq!(T::MAX.saturating_sub_unsigned(T::MIN.cast_unsigned()), T::N_1);
    assert_eq!(T::MAX.saturating_sub_unsigned(T::MAX.cast_unsigned()), T::P_0);
  });

  test!(@sint, test_saturating_mul, () => {
    assert_eq!(T::MIN.saturating_mul(T::N_1), T::MAX);
    assert_eq!(T::MIN.saturating_mul(T::P_1), T::MIN);
    assert_eq!(T::MIN.saturating_mul(T::MIN), T::MAX);
    assert_eq!(T::MIN.saturating_mul(T::MAX), T::MIN);

    assert_eq!(T::MAX.saturating_mul(T::N_1), -T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::P_1), T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::MIN), T::MIN);
    assert_eq!(T::MAX.saturating_mul(T::MAX), T::MAX);
  });

  test!(@sint, test_saturating_div, () => {
    assert_eq!(T::N_1.saturating_div(T::N_1), T::P_1);
    assert_eq!(T::P_0.saturating_div(T::N_1), T::P_0);
    assert_eq!(T::P_1.saturating_div(T::N_1), T::N_1);
    assert_eq!(T::MIN.saturating_div(T::N_1), T::MAX);
    assert_eq!(T::MAX.saturating_div(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.saturating_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.saturating_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.saturating_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.saturating_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.saturating_div(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_saturating_neg, () => {
    assert_eq!(T::N_1.saturating_neg(), T::P_1);
    assert_eq!(T::P_0.saturating_neg(), T::P_0);
    assert_eq!(T::P_1.saturating_neg(), T::N_1);
    assert_eq!(T::MIN.saturating_neg(), T::MAX);
    assert_eq!(T::MAX.saturating_neg(), T::MIN + T::P_1);
  });

  test!(@sint, test_saturating_pow, () => {
    assert_eq!(T::N_1.saturating_pow(2), T::P_1);
    assert_eq!(T::P_0.saturating_pow(2), T::P_0);
    assert_eq!(T::P_1.saturating_pow(2), T::P_1);
    assert_eq!(T::MIN.saturating_pow(2), T::MAX);
    assert_eq!(T::MAX.saturating_pow(2), T::MAX);
  });

  test!(@sint, test_saturating_abs, () => {
    assert_eq!(T::N_1.saturating_abs(), T::P_1);
    assert_eq!(T::P_0.saturating_abs(), T::P_0);
    assert_eq!(T::P_1.saturating_abs(), T::P_1);
    assert_eq!(T::MIN.saturating_abs(), T::MAX);
    assert_eq!(T::MAX.saturating_abs(), T::MAX);
  });
}
