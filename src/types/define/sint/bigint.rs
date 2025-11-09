#![allow(unused_imports, reason = "feature-dependant")]

use crate::llapi;
use crate::types::int;
use crate::types::uint;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "carrying_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_add(rhs);
    let (c, d): (Self, bool) = a.overflowing_add(Self::from_bool(carry));

    (c, b != d)
  }

  #[doc = include_doc!(int, "borrowing_sub")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_sub(rhs);
    let (c, d): (Self, bool) = a.overflowing_sub(Self::from_bool(borrow));

    (c, b != d)
  }

  #[doc = include_doc!(int, "carrying_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (uint<N>, Self) {
    Self::carrying_mul_add(self, rhs, carry, Self::ZERO)
  }

  #[doc = include_doc!(int, "carrying_mul_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (uint<N>, Self) {
    llapi::carrying_smul_sadd::<Self, uint<N>, N>(self, rhs, carry, add)
  }

  #[doc = include_doc!(int, "widening_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn widening_mul(self, rhs: Self) -> (uint<N>, Self) {
    Self::carrying_mul_add(self, rhs, Self::ZERO, Self::ZERO)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_carrying_add, () => {
    assert_eq!(T::MIN.carrying_add(T::N_1, false), (T::MAX, true));
    assert_eq!(T::MIN.carrying_add(T::N_1, true), (T::MIN, false));
    assert_eq!(T::MIN.carrying_add(T::P_1, false), (T::MIN + T::P_1, false));
    assert_eq!(T::MIN.carrying_add(T::P_1, true), (T::MIN + T::P_2, false));
    assert_eq!(T::MIN.carrying_add(T::MIN, false), (T::P_0, true));
    assert_eq!(T::MIN.carrying_add(T::MIN, true), (T::P_1, true));
    assert_eq!(T::MIN.carrying_add(T::MAX, false), (T::N_1, false));
    assert_eq!(T::MIN.carrying_add(T::MAX, true), (T::P_0, false));

    assert_eq!(T::MAX.carrying_add(T::N_1, false), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.carrying_add(T::N_1, true), (T::MAX, false));
    assert_eq!(T::MAX.carrying_add(T::P_1, false), (T::MIN, true));
    assert_eq!(T::MAX.carrying_add(T::P_1, true), (T::MIN + T::P_1, true));
    assert_eq!(T::MAX.carrying_add(T::MIN, false), (T::N_1, false));
    assert_eq!(T::MAX.carrying_add(T::MIN, true), (T::P_0, false));
    assert_eq!(T::MAX.carrying_add(T::MAX, false), (T::N_2, true));
    assert_eq!(T::MAX.carrying_add(T::MAX, true), (T::N_1, true));
  });

  test!(@sint, test_borrowing_sub, () => {
    assert_eq!(T::MIN.borrowing_sub(T::N_1, false), (T::MIN + T::P_1, false));
    assert_eq!(T::MIN.borrowing_sub(T::N_1, true), (T::MIN, false));
    assert_eq!(T::MIN.borrowing_sub(T::P_1, false), (T::MAX, true));
    assert_eq!(T::MIN.borrowing_sub(T::P_1, true), (T::MAX - T::P_1, true));
    assert_eq!(T::MIN.borrowing_sub(T::MIN, false), (T::P_0, false));
    assert_eq!(T::MIN.borrowing_sub(T::MIN, true), (T::N_1, false));
    assert_eq!(T::MIN.borrowing_sub(T::MAX, false), (T::P_1, true));
    assert_eq!(T::MIN.borrowing_sub(T::MAX, true), (T::P_0, true));

    assert_eq!(T::MAX.borrowing_sub(T::N_1, false), (T::MIN, true));
    assert_eq!(T::MAX.borrowing_sub(T::N_1, true), (T::MAX, false));
    assert_eq!(T::MAX.borrowing_sub(T::P_1, false), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.borrowing_sub(T::P_1, true), (T::MAX - T::P_2, false));
    assert_eq!(T::MAX.borrowing_sub(T::MIN, false), (T::N_1, true));
    assert_eq!(T::MAX.borrowing_sub(T::MIN, true), (T::N_2, true));
    assert_eq!(T::MAX.borrowing_sub(T::MAX, false), (T::P_0, false));
    assert_eq!(T::MAX.borrowing_sub(T::MAX, true), (T::N_1, false));
  });

  test!(@sint, test_carrying_mul, () => {
    assert_eq!(T::MAX.carrying_mul(T::MAX, T::P_0), (U::P_1, T::MAX / T::P_2));
    assert_eq!(T::MAX.carrying_mul(T::MAX, T::MAX), (U::MAX / U::P_2 + U::P_1, T::MAX / T::P_2));
    assert_eq!(T::MAX.carrying_mul(T::MAX, T::MIN), (U::MAX / U::P_2 + U::P_2, T::MAX / T::P_2 - T::P_1));
    assert_eq!(T::MIN.carrying_mul(T::MAX, T::P_0), (T::MIN.cast_unsigned(), T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul(T::MAX, T::MAX), (U::MAX, T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul(T::MAX, T::MIN), (U::P_0, T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul(T::MIN, T::P_0), (U::P_0, T::MAX / T::P_2 + T::P_1));
    assert_eq!(T::MIN.carrying_mul(T::MIN, T::MAX), (U::MAX / U::P_2, T::MAX / T::P_2 + T::P_1));
    assert_eq!(T::MIN.carrying_mul(T::MIN, T::MIN), (U::MAX / U::P_2 + U::P_1, T::MAX / T::P_2));
  });

  test!(@sint, test_carrying_mul_add, () => {
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::P_0, T::P_0), (U::P_1, T::MAX / T::P_2));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MAX, T::P_0), (U::MAX / U::P_2 + U::P_1, T::MAX / T::P_2));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MIN, T::P_0), (U::MAX / U::P_2 + U::P_2, T::MAX / T::P_2 - T::P_1));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MAX, T::MAX), (U::MAX, T::MAX / T::P_2));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MAX, T::MIN), (U::P_0, T::MAX / T::P_2));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MIN, T::MIN), (U::P_1, T::MAX / T::P_2 - T::P_1));
    assert_eq!(T::MIN.carrying_mul_add(T::MAX, T::P_0, T::P_0), (T::MIN.cast_unsigned(), T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul_add(T::MAX, T::MAX, T::P_0), (U::MAX, T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul_add(T::MAX, T::MIN, T::P_0), (U::P_0, T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul_add(T::MAX, T::MAX, T::MAX), (U::MAX / U::P_2 - U::P_1, T::MIN / T::P_2 + T::P_1));
    assert_eq!(T::MIN.carrying_mul_add(T::MAX, T::MAX, T::MIN), (U::MAX / U::P_2, T::MIN / T::P_2));
    assert_eq!(T::MIN.carrying_mul_add(T::MAX, T::MIN, T::MIN), (U::MAX / U::P_2 + U::P_1, T::MIN / T::P_2 - T::P_1));
    assert_eq!(T::MIN.carrying_mul_add(T::MIN, T::P_0, T::P_0), (U::P_0, T::MAX / T::P_2 + T::P_1));
    assert_eq!(T::MIN.carrying_mul_add(T::MIN, T::MAX, T::P_0), (U::MAX / U::P_2, T::MAX / T::P_2 + T::P_1));
    assert_eq!(T::MIN.carrying_mul_add(T::MIN, T::MIN, T::P_0), (U::MAX / U::P_2 + U::P_1, T::MAX / T::P_2));
    assert_eq!(T::MIN.carrying_mul_add(T::MIN, T::MAX, T::MAX), (U::MAX - U::P_1, T::MAX / T::P_2 + T::P_1));
    assert_eq!(T::MIN.carrying_mul_add(T::MIN, T::MAX, T::MIN), (U::MAX, T::MAX / T::P_2));
    assert_eq!(T::MIN.carrying_mul_add(T::MIN, T::MIN, T::MIN), (U::P_0, T::MAX / T::P_2));
  });

  test!(@sint, test_widening_mul, () => {
    assert_eq!(T::MAX.widening_mul(T::MAX), (U::P_1, T::MAX / T::P_2));
    assert_eq!(T::MIN.widening_mul(T::MAX), (T::MIN.cast_unsigned(), T::MIN / T::P_2));
    assert_eq!(T::MIN.widening_mul(T::MIN), (U::P_0, T::MAX / T::P_2 + T::P_1));
  });
}
