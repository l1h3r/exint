use ::core::option::Option::None;
use ::core::option::Option::Some;

use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "saturating_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add(self, rhs: Self) -> Self {
    llapi::saturating_uadd::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "saturating_add_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_add_signed(self, rhs: int<N>) -> Self {
    let (result, overflow): (Self, bool) = self.overflowing_add(rhs.cast_unsigned());

    if overflow == rhs.is_negative() {
      result
    } else if overflow {
      Self::MAX
    } else {
      Self::ZERO
    }
  }

  #[doc = include_doc!(uint, "saturating_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub(self, rhs: Self) -> Self {
    llapi::saturating_usub::<Self, N>(self, rhs)
  }

  #[doc = include_doc!(uint, "saturating_sub_signed")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_sub_signed(self, rhs: int<N>) -> Self {
    let (result, overflow): (Self, bool) = self.overflowing_sub_signed(rhs);

    if !overflow {
      result
    } else if rhs.is_negative() {
      Self::MAX
    } else {
      Self::ZERO
    }
  }

  #[doc = include_doc!(uint, "saturating_mul")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_mul(self, rhs: Self) -> Self {
    match self.checked_mul(rhs) {
      Some(result) => result,
      None => Self::MAX,
    }
  }

  #[doc = include_doc!(uint, "saturating_div")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_div(self, rhs: Self) -> Self {
    self.wrapping_div(rhs)
  }

  #[doc = include_doc!(uint, "saturating_pow")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn saturating_pow(self, exp: u32) -> Self {
    match self.checked_pow(exp) {
      Some(result) => result,
      None => Self::MAX,
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_saturating_add, () => {
    assert_eq!(T::MIN.saturating_add(T::P_1), T::P_1);
    assert_eq!(T::MIN.saturating_add(T::P_2), T::P_2);
    assert_eq!(T::MIN.saturating_add(T::MIN), T::MIN);
    assert_eq!(T::MIN.saturating_add(T::MAX), T::MAX);

    assert_eq!(T::MAX.saturating_add(T::P_1), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::P_2), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::MIN), T::MAX);
    assert_eq!(T::MAX.saturating_add(T::MAX), T::MAX);
  });

  test!(@uint, test_saturating_add_signed, () => {
    assert_eq!(T::MIN.saturating_add_signed(T::P_1.cast_signed()), T::P_1);
    assert_eq!(T::MIN.saturating_add_signed(T::P_2.cast_signed()), T::P_2);
    assert_eq!(T::MIN.saturating_add_signed(T::MIN.cast_signed()), T::MIN);
    assert_eq!(T::MIN.saturating_add_signed(T::MAX.cast_signed()), T::MIN);

    assert_eq!(T::MAX.saturating_add_signed(T::P_1.cast_signed()), T::MAX);
    assert_eq!(T::MAX.saturating_add_signed(T::P_2.cast_signed()), T::MAX);
    assert_eq!(T::MAX.saturating_add_signed(T::MIN.cast_signed()), T::MAX);
    assert_eq!(T::MAX.saturating_add_signed(T::MAX.cast_signed()), T::MAX - T::P_1);
  });

  test!(@uint, test_saturating_sub, () => {
    assert_eq!(T::MIN.saturating_sub(T::P_1), T::MIN);
    assert_eq!(T::MIN.saturating_sub(T::P_2), T::MIN);
    assert_eq!(T::MIN.saturating_sub(T::MIN), T::MIN);
    assert_eq!(T::MIN.saturating_sub(T::MAX), T::MIN);

    assert_eq!(T::MAX.saturating_sub(T::P_1), T::MAX - T::P_1);
    assert_eq!(T::MAX.saturating_sub(T::P_2), T::MAX - T::P_2);
    assert_eq!(T::MAX.saturating_sub(T::MIN), T::MAX);
    assert_eq!(T::MAX.saturating_sub(T::MAX), T::MIN);
  });

  test!(@uint, test_saturating_sub_signed, () => {
    assert_eq!(T::MIN.saturating_sub_signed(T::P_1.cast_signed()), T::MIN);
    assert_eq!(T::MIN.saturating_sub_signed(T::P_2.cast_signed()), T::MIN);
    assert_eq!(T::MIN.saturating_sub_signed(T::MIN.cast_signed()), T::MIN);
    assert_eq!(T::MIN.saturating_sub_signed(T::MAX.cast_signed()), T::P_1);

    assert_eq!(T::MAX.saturating_sub_signed(T::P_1.cast_signed()), T::MAX - T::P_1);
    assert_eq!(T::MAX.saturating_sub_signed(T::P_2.cast_signed()), T::MAX - T::P_2);
    assert_eq!(T::MAX.saturating_sub_signed(T::MIN.cast_signed()), T::MAX);
    assert_eq!(T::MAX.saturating_sub_signed(T::MAX.cast_signed()), T::MAX);
  });

  test!(@uint, test_saturating_mul, () => {
    assert_eq!(T::MIN.saturating_mul(T::P_1), T::MIN);
    assert_eq!(T::MIN.saturating_mul(T::P_2), T::MIN);
    assert_eq!(T::MIN.saturating_mul(T::MIN), T::MIN);
    assert_eq!(T::MIN.saturating_mul(T::MAX), T::MIN);

    assert_eq!(T::MAX.saturating_mul(T::P_1), T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::P_2), T::MAX);
    assert_eq!(T::MAX.saturating_mul(T::MIN), T::MIN);
    assert_eq!(T::MAX.saturating_mul(T::MAX), T::MAX);
  });

  test!(@uint, test_saturating_div, () => {
    // Forwards to wrapping_div
  });

  test!(@uint, test_saturating_pow, () => {
    assert_eq!(T::P_1.saturating_pow(2), T::P_1);
    assert_eq!(T::P_2.saturating_pow(2), T::P_2 + T::P_2);
    assert_eq!(T::MIN.saturating_pow(2), T::MIN);
    assert_eq!(T::MAX.saturating_pow(2), T::MAX);
  });
}
