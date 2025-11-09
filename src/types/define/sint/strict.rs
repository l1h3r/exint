use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::panic;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "strict_add")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_add(self, rhs: Self) -> Self {
    match self.checked_add(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(int, "strict_add_unsigned")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_add_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_add_unsigned(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(int, "strict_sub")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_sub(self, rhs: Self) -> Self {
    match self.checked_sub(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(int, "strict_sub_unsigned")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_sub_unsigned(self, rhs: uint<N>) -> Self {
    match self.checked_sub_unsigned(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(int, "strict_mul")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None => panic::mul(),
    }
  }

  #[doc = include_doc!(int, "strict_div")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_div(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(int, "strict_div_euclid")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_div_euclid(self, rhs: Self) -> Self {
    match self.checked_div_euclid(rhs) {
      Some(result) => result,
      None if rhs.is_zero() => panic::div_zero(),
      None => panic::div(),
    }
  }

  #[doc = include_doc!(int, "strict_rem")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_rem(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(int, "strict_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
    match self.checked_rem_euclid(rhs) {
      Some(result) => result,
      None if rhs.is_zero() => panic::rem_zero(),
      None => panic::rem(),
    }
  }

  #[doc = include_doc!(int, "strict_shl")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_shl(self, rhs: u32) -> Self {
    match self.checked_shl(rhs) {
      Some(result) => result,
      None => panic::shl(),
    }
  }

  #[doc = include_doc!(int, "strict_shr")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_shr(self, rhs: u32) -> Self {
    match self.checked_shr(rhs) {
      Some(result) => result,
      None => panic::shr(),
    }
  }

  #[doc = include_doc!(int, "strict_neg")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_neg(self) -> Self {
    match self.checked_neg() {
      Some(result) => result,
      None => panic::neg(),
    }
  }

  #[doc = include_doc!(int, "strict_pow")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None => panic::mul(),
    }
  }

  #[doc = include_doc!(int, "strict_abs")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_abs(self) -> Self {
    if self.is_negative() {
      self.strict_neg()
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

  test!(@sint, test_strict_add, () => {
    assert_panic!(T::MIN.strict_add(T::N_1), message = ADD);
    assert_eq!(T::MIN.strict_add(T::P_1), T::MIN + T::P_1);
    assert_panic!(T::MIN.strict_add(T::MIN), message = ADD);
    assert_eq!(T::MIN.strict_add(T::MAX), T::N_1);

    assert_eq!(T::MAX.strict_add(T::N_1), T::MAX - T::P_1);
    assert_panic!(T::MAX.strict_add(T::P_1), message = ADD);
    assert_eq!(T::MAX.strict_add(T::MIN), T::N_1);
    assert_panic!(T::MAX.strict_add(T::MAX), message = ADD);
  });

  test!(@sint, test_strict_add_unsigned, () => {
    assert_eq!(T::MIN.strict_add_unsigned(T::N_1.cast_unsigned()), T::MAX);
    assert_eq!(T::MIN.strict_add_unsigned(T::P_1.cast_unsigned()), T::MIN + T::P_1);
    assert_eq!(T::MIN.strict_add_unsigned(T::MIN.cast_unsigned()), T::P_0);
    assert_eq!(T::MIN.strict_add_unsigned(T::MAX.cast_unsigned()), T::N_1);

    assert_panic!(T::MAX.strict_add_unsigned(T::N_1.cast_unsigned()), message = ADD);
    assert_panic!(T::MAX.strict_add_unsigned(T::P_1.cast_unsigned()), message = ADD);
    assert_panic!(T::MAX.strict_add_unsigned(T::MIN.cast_unsigned()), message = ADD);
    assert_panic!(T::MAX.strict_add_unsigned(T::MAX.cast_unsigned()), message = ADD);
  });

  test!(@sint, test_strict_sub, () => {
    assert_eq!(T::MIN.strict_sub(T::N_1), T::MIN + T::P_1);
    assert_panic!(T::MIN.strict_sub(T::P_1), message = SUB);
    assert_eq!(T::MIN.strict_sub(T::MIN), T::P_0);
    assert_panic!(T::MIN.strict_sub(T::MAX), message = SUB);

    assert_panic!(T::MAX.strict_sub(T::N_1), message = SUB);
    assert_eq!(T::MAX.strict_sub(T::P_1), T::MAX - T::P_1);
    assert_panic!(T::MAX.strict_sub(T::MIN), message = SUB);
    assert_eq!(T::MAX.strict_sub(T::MAX), T::P_0);
  });

  test!(@sint, test_strict_sub_unsigned, () => {
    assert_panic!(T::MIN.strict_sub_unsigned(T::N_1.cast_unsigned()), message = SUB);
    assert_panic!(T::MIN.strict_sub_unsigned(T::P_1.cast_unsigned()), message = SUB);
    assert_panic!(T::MIN.strict_sub_unsigned(T::MIN.cast_unsigned()), message = SUB);
    assert_panic!(T::MIN.strict_sub_unsigned(T::MAX.cast_unsigned()), message = SUB);

    assert_eq!(T::MAX.strict_sub_unsigned(T::N_1.cast_unsigned()), T::MIN);
    assert_eq!(T::MAX.strict_sub_unsigned(T::P_1.cast_unsigned()), T::MAX - T::P_1);
    assert_eq!(T::MAX.strict_sub_unsigned(T::MIN.cast_unsigned()), T::N_1);
    assert_eq!(T::MAX.strict_sub_unsigned(T::MAX.cast_unsigned()), T::P_0);
  });

  test!(@sint, test_strict_mul, () => {
    assert_panic!(T::MIN.strict_mul(T::N_1), message = MUL);
    assert_eq!(T::MIN.strict_mul(T::P_1), T::MIN);
    assert_panic!(T::MIN.strict_mul(T::MIN), message = MUL);
    assert_panic!(T::MIN.strict_mul(T::MAX), message = MUL);

    assert_eq!(T::MAX.strict_mul(T::N_1), -T::MAX);
    assert_eq!(T::MAX.strict_mul(T::P_1), T::MAX);
    assert_panic!(T::MAX.strict_mul(T::MIN), message = MUL);
    assert_panic!(T::MAX.strict_mul(T::MAX), message = MUL);
  });

  test!(@sint, test_strict_div, () => {
    assert_eq!(T::N_1.strict_div(T::N_1), T::P_1);
    assert_eq!(T::P_0.strict_div(T::N_1), T::P_0);
    assert_eq!(T::P_1.strict_div(T::N_1), T::N_1);
    assert_panic!(T::MIN.strict_div(T::N_1), message = DIV);
    assert_eq!(T::MAX.strict_div(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.strict_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.strict_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.strict_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.strict_div(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.strict_div(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_strict_div_euclid, () => {
    assert_eq!(T::N_1.strict_div_euclid(T::N_1), T::P_1);
    assert_eq!(T::P_0.strict_div_euclid(T::N_1), T::P_0);
    assert_eq!(T::P_1.strict_div_euclid(T::N_1), T::N_1);
    assert_panic!(T::MIN.strict_div_euclid(T::N_1), message = DIV);
    assert_eq!(T::MAX.strict_div_euclid(T::N_1), T::MIN + T::P_1);

    assert_panic!(T::N_1.strict_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_0.strict_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::P_1.strict_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MIN.strict_div_euclid(T::P_0), message = DIV_ZERO);
    assert_panic!(T::MAX.strict_div_euclid(T::P_0), message = DIV_ZERO);
  });

  test!(@sint, test_strict_rem, () => {
    assert_eq!(T::N_1.strict_rem(T::N_1), T::P_0);
    assert_eq!(T::P_0.strict_rem(T::N_1), T::P_0);
    assert_eq!(T::P_1.strict_rem(T::N_1), T::P_0);
    assert_panic!(T::MIN.strict_rem(T::N_1), message = REM);
    assert_eq!(T::MAX.strict_rem(T::N_1), T::P_0);

    assert_panic!(T::N_1.strict_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_0.strict_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_1.strict_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::MIN.strict_rem(T::P_0), message = REM_ZERO);
    assert_panic!(T::MAX.strict_rem(T::P_0), message = REM_ZERO);
  });

  test!(@sint, test_strict_rem_euclid, () => {
    assert_eq!(T::N_1.strict_rem_euclid(T::N_1), T::P_0);
    assert_eq!(T::P_0.strict_rem_euclid(T::N_1), T::P_0);
    assert_eq!(T::P_1.strict_rem_euclid(T::N_1), T::P_0);
    assert_panic!(T::MIN.strict_rem_euclid(T::N_1), message = REM);
    assert_eq!(T::MAX.strict_rem_euclid(T::N_1), T::P_0);

    assert_panic!(T::N_1.strict_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_0.strict_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::P_1.strict_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::MIN.strict_rem_euclid(T::P_0), message = REM_ZERO);
    assert_panic!(T::MAX.strict_rem_euclid(T::P_0), message = REM_ZERO);
  });

  test!(@sint, test_strict_shl, () => {
    assert_strict_shift!(strict_shl, T, T::N_1);
    assert_strict_shift!(strict_shl, T, T::P_1);
    assert_strict_shift!(strict_shl, T, T::P_8);
    assert_strict_shift!(strict_shl, T, T::P_17);
    assert_strict_shift!(strict_shl, T, T::MIN);
    assert_strict_shift!(strict_shl, T, T::MAX);
  });

  test!(@sint, test_strict_shr, () => {
    assert_strict_shift!(strict_shr, T, T::N_1);
    assert_strict_shift!(strict_shr, T, T::P_1);
    assert_strict_shift!(strict_shr, T, T::P_8);
    assert_strict_shift!(strict_shr, T, T::P_17);
    assert_strict_shift!(strict_shr, T, T::MIN);
    assert_strict_shift!(strict_shr, T, T::MAX);
  });

  test!(@sint, test_strict_neg, () => {
    assert_eq!(T::N_1.strict_neg(), T::P_1);
    assert_eq!(T::P_0.strict_neg(), T::P_0);
    assert_eq!(T::P_1.strict_neg(), T::N_1);
    assert_panic!(T::MIN.strict_neg(), message = NEG);
    assert_eq!(T::MAX.strict_neg(), T::MIN + T::P_1);
  });

  test!(@sint, test_strict_pow, () => {
    assert_eq!(T::N_1.strict_pow(2), T::P_1);
    assert_eq!(T::P_0.strict_pow(2), T::P_0);
    assert_eq!(T::P_1.strict_pow(2), T::P_1);
    assert_panic!(T::MIN.strict_pow(2), message = MUL);
    assert_panic!(T::MAX.strict_pow(2), message = MUL);
  });

  test!(@sint, test_strict_abs, () => {
    assert_eq!(T::N_1.strict_abs(), T::P_1);
    assert_eq!(T::P_0.strict_abs(), T::P_0);
    assert_eq!(T::P_1.strict_abs(), T::P_1);
    assert_panic!(T::MIN.strict_abs(), message = NEG);
    assert_eq!(T::MAX.strict_abs(), T::MAX);
  });
}
