use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::panic;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "strict_add")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_add(self, rhs: Self) -> Self {
    match self.checked_add(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(uint, "strict_add_signed")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_add_signed(self, rhs: int<N>) -> Self {
    match self.checked_add_signed(rhs) {
      Some(result) => result,
      None => panic::add(),
    }
  }

  #[doc = include_doc!(uint, "strict_sub")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_sub(self, rhs: Self) -> Self {
    match self.checked_sub(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(uint, "strict_sub_signed")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_sub_signed(self, rhs: int<N>) -> Self {
    match self.checked_sub_signed(rhs) {
      Some(result) => result,
      None => panic::sub(),
    }
  }

  #[doc = include_doc!(uint, "strict_mul")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None => panic::mul(),
    }
  }

  #[doc = include_doc!(uint, "strict_div")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_div(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "strict_div_euclid")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_div_euclid(self, rhs: Self) -> Self {
    self.const_div(rhs)
  }

  #[doc = include_doc!(uint, "strict_rem")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_rem(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "strict_rem_euclid")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
    self.const_rem(rhs)
  }

  #[doc = include_doc!(uint, "strict_shl")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_shl(self, rhs: u32) -> Self {
    match self.checked_shl(rhs) {
      Some(result) => result,
      None => panic::shl(),
    }
  }

  #[doc = include_doc!(uint, "strict_shr")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_shr(self, rhs: u32) -> Self {
    match self.checked_shr(rhs) {
      Some(result) => result,
      None => panic::shr(),
    }
  }

  #[doc = include_doc!(uint, "strict_neg")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_neg(self) -> Self {
    match self.checked_neg() {
      Some(result) => result,
      None => panic::neg(),
    }
  }

  #[doc = include_doc!(uint, "strict_pow")]
  #[must_use = must_use_doc!()]
  #[track_caller]
  #[inline]
  pub const fn strict_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None => panic::mul(),
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_strict_add, () => {
    assert_eq!(T::MIN.strict_add(T::P_1), T::P_1);
    assert_eq!(T::MIN.strict_add(T::P_2), T::P_2);
    assert_eq!(T::MIN.strict_add(T::MIN), T::MIN);
    assert_eq!(T::MIN.strict_add(T::MAX), T::MAX);

    assert_panic!(T::MAX.strict_add(T::P_1), message = ADD);
    assert_panic!(T::MAX.strict_add(T::P_2), message = ADD);
    assert_eq!(T::MAX.strict_add(T::MIN), T::MAX);
    assert_panic!(T::MAX.strict_add(T::MAX), message = ADD);
  });

  test!(@uint, test_strict_add_signed, () => {
    assert_eq!(T::MIN.strict_add_signed(T::P_1.cast_signed()), T::P_1);
    assert_eq!(T::MIN.strict_add_signed(T::P_2.cast_signed()), T::P_2);
    assert_eq!(T::MIN.strict_add_signed(T::MIN.cast_signed()), T::MIN);
    assert_panic!(T::MIN.strict_add_signed(T::MAX.cast_signed()), message = ADD);

    assert_panic!(T::MAX.strict_add_signed(T::P_1.cast_signed()), message = ADD);
    assert_panic!(T::MAX.strict_add_signed(T::P_2.cast_signed()), message = ADD);
    assert_eq!(T::MAX.strict_add_signed(T::MIN.cast_signed()), T::MAX);
    assert_eq!(T::MAX.strict_add_signed(T::MAX.cast_signed()), T::MAX - T::P_1);
  });

  test!(@uint, test_strict_sub, () => {
    assert_panic!(T::MIN.strict_sub(T::P_1), message = SUB);
    assert_panic!(T::MIN.strict_sub(T::P_2), message = SUB);
    assert_eq!(T::MIN.strict_sub(T::MIN), T::MIN);
    assert_panic!(T::MIN.strict_sub(T::MAX), message = SUB);

    assert_eq!(T::MAX.strict_sub(T::P_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.strict_sub(T::P_2), T::MAX - T::P_2);
    assert_eq!(T::MAX.strict_sub(T::MIN), T::MAX);
    assert_eq!(T::MAX.strict_sub(T::MAX), T::MIN);
  });

  test!(@uint, test_strict_sub_signed, () => {
    assert_panic!(T::MIN.strict_sub_signed(T::P_1.cast_signed()), message = SUB);
    assert_panic!(T::MIN.strict_sub_signed(T::P_2.cast_signed()), message = SUB);
    assert_eq!(T::MIN.strict_sub_signed(T::MIN.cast_signed()), T::MIN);
    assert_eq!(T::MIN.strict_sub_signed(T::MAX.cast_signed()), T::P_1);

    assert_eq!(T::MAX.strict_sub_signed(T::P_1.cast_signed()), T::MAX - T::P_1);
    assert_eq!(T::MAX.strict_sub_signed(T::P_2.cast_signed()), T::MAX - T::P_2);
    assert_eq!(T::MAX.strict_sub_signed(T::MIN.cast_signed()), T::MAX);
    assert_panic!(T::MAX.strict_sub_signed(T::MAX.cast_signed()), message = SUB);
  });

  test!(@uint, test_strict_mul, () => {
    assert_eq!(T::MIN.strict_mul(T::P_1), T::MIN);
    assert_eq!(T::MIN.strict_mul(T::P_2), T::MIN);
    assert_eq!(T::MIN.strict_mul(T::MIN), T::MIN);
    assert_eq!(T::MIN.strict_mul(T::MAX), T::MIN);

    assert_eq!(T::MAX.strict_mul(T::P_1), T::MAX);
    assert_panic!(T::MAX.strict_mul(T::P_2), message = MUL);
    assert_eq!(T::MAX.strict_mul(T::MIN), T::MIN);
    assert_panic!(T::MAX.strict_mul(T::MAX), message = MUL);
  });

  test!(@uint, test_strict_div, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_strict_div_euclid, () => {
    // Forwards to Div::div
  });

  test!(@uint, test_strict_rem, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_strict_rem_euclid, () => {
    // Forwards to Rem::rem
  });

  test!(@uint, test_strict_shl, () => {
    assert_strict_shift!(strict_shl, T, T::P_1);
    assert_strict_shift!(strict_shl, T, T::P_8);
    assert_strict_shift!(strict_shl, T, T::P_17);
    assert_strict_shift!(strict_shl, T, T::MIN);
    assert_strict_shift!(strict_shl, T, T::MAX);
  });

  test!(@uint, test_strict_shr, () => {
    assert_strict_shift!(strict_shr, T, T::P_1);
    assert_strict_shift!(strict_shr, T, T::P_8);
    assert_strict_shift!(strict_shr, T, T::P_17);
    assert_strict_shift!(strict_shr, T, T::MIN);
    assert_strict_shift!(strict_shr, T, T::MAX);
  });

  test!(@uint, test_strict_neg, () => {
    assert_panic!(T::P_1.strict_neg(), message = NEG);
    assert_panic!(T::P_2.strict_neg(), message = NEG);
    assert_eq!(T::MIN.strict_neg(), T::MIN);
    assert_panic!(T::MAX.strict_neg(), message = NEG);
  });

  test!(@uint, test_strict_pow, () => {
    assert_eq!(T::P_1.strict_pow(2), T::P_1);
    assert_eq!(T::P_2.strict_pow(2), T::P_2 + T::P_2);
    assert_eq!(T::MIN.strict_pow(2), T::MIN);
    assert_panic!(T::MAX.strict_pow(2), message = MUL);
  });
}
