#![allow(unused_imports, reason = "feature-dependant")]

use crate::llapi;
use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "carrying_add")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_add(rhs);
    let (c, d): (Self, bool) = a.overflowing_add(Self::from_bool(carry));

    (c, b | d)
  }

  #[doc = include_doc!(uint, "borrowing_sub")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
    let (a, b): (Self, bool) = self.overflowing_sub(rhs);
    let (c, d): (Self, bool) = a.overflowing_sub(Self::from_bool(borrow));

    (c, b | d)
  }

  #[doc = include_doc!(uint, "carrying_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
    Self::carrying_mul_add(self, rhs, carry, Self::ZERO)
  }

  #[doc = include_doc!(uint, "carrying_mul_add")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Self, Self) {
    llapi::carrying_umul_uadd::<Self, N>(self, rhs, carry, add)
  }

  #[doc = include_doc!(uint, "widening_mul")]
  #[cfg(feature = "bigint_helper_methods")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
    Self::carrying_mul_add(self, rhs, Self::ZERO, Self::ZERO)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_carrying_add, () => {
    assert_eq!(T::MIN.carrying_add(T::P_1, false), (T::P_1, false));
    assert_eq!(T::MIN.carrying_add(T::P_1, true), (T::P_2, false));
    assert_eq!(T::MIN.carrying_add(T::P_2, false), (T::P_2, false));
    assert_eq!(T::MIN.carrying_add(T::P_2, true), (T::P_3, false));
    assert_eq!(T::MIN.carrying_add(T::MIN, false), (T::MIN, false));
    assert_eq!(T::MIN.carrying_add(T::MIN, true), (T::P_1, false));
    assert_eq!(T::MIN.carrying_add(T::MAX, false), (T::MAX, false));
    assert_eq!(T::MIN.carrying_add(T::MAX, true), (T::MIN, true));

    assert_eq!(T::MAX.carrying_add(T::P_1, false), (T::MIN, true));
    assert_eq!(T::MAX.carrying_add(T::P_1, true), (T::P_1, true));
    assert_eq!(T::MAX.carrying_add(T::P_2, false), (T::P_1, true));
    assert_eq!(T::MAX.carrying_add(T::P_2, true), (T::P_2, true));
    assert_eq!(T::MAX.carrying_add(T::MIN, false), (T::MAX, false));
    assert_eq!(T::MAX.carrying_add(T::MIN, true), (T::MIN, true));
    assert_eq!(T::MAX.carrying_add(T::MAX, false), (T::MAX - T::P_1, true));
    assert_eq!(T::MAX.carrying_add(T::MAX, true), (T::MAX, true));
  });

  test!(@uint, test_borrowing_sub, () => {
    assert_eq!(T::MIN.borrowing_sub(T::P_1, false), (T::MAX, true));
    assert_eq!(T::MIN.borrowing_sub(T::P_1, true), (T::MAX - T::P_1, true));
    assert_eq!(T::MIN.borrowing_sub(T::P_2, false), (T::MAX - T::P_1, true));
    assert_eq!(T::MIN.borrowing_sub(T::P_2, true), (T::MAX - T::P_2, true));
    assert_eq!(T::MIN.borrowing_sub(T::MIN, false), (T::MIN, false));
    assert_eq!(T::MIN.borrowing_sub(T::MIN, true), (T::MAX, true));
    assert_eq!(T::MIN.borrowing_sub(T::MAX, false), (T::P_1, true));
    assert_eq!(T::MIN.borrowing_sub(T::MAX, true), (T::MIN, true));

    assert_eq!(T::MAX.borrowing_sub(T::P_1, false), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.borrowing_sub(T::P_1, true), (T::MAX - T::P_2, false));
    assert_eq!(T::MAX.borrowing_sub(T::P_2, false), (T::MAX - T::P_2, false));
    assert_eq!(T::MAX.borrowing_sub(T::P_2, true), (T::MAX - T::P_3, false));
    assert_eq!(T::MAX.borrowing_sub(T::MIN, false), (T::MAX, false));
    assert_eq!(T::MAX.borrowing_sub(T::MIN, true), (T::MAX - T::P_1, false));
    assert_eq!(T::MAX.borrowing_sub(T::MAX, false), (T::MIN, false));
    assert_eq!(T::MAX.borrowing_sub(T::MAX, true), (T::MAX, true));
  });

  test!(@uint, test_carrying_mul, () => {
    assert_eq!(T::MAX.carrying_mul(T::MAX, T::P_0), (T::P_1, T::MAX - T::P_1));
    assert_eq!(T::MAX.carrying_mul(T::MAX, T::MAX), (T::P_0, T::MAX));
  });

  test!(@uint, test_carrying_mul_add, () => {
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::P_0, T::P_0), (T::P_1, T::MAX - T::P_1));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MAX, T::P_0), (T::P_0, T::MAX));
    assert_eq!(T::MAX.carrying_mul_add(T::MAX, T::MAX, T::MAX), (T::MAX, T::MAX));
  });

  test!(@uint, test_widening_mul, () => {
    assert_eq!(T::MAX.widening_mul(T::MAX), (T::P_1, T::MAX - T::P_1));
  });
}
