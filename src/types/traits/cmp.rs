use ::core::cmp::Eq;
use ::core::cmp::Ord;
use ::core::cmp::Ordering;
use ::core::cmp::PartialEq;
use ::core::cmp::PartialOrd;
use ::core::option::Option;
use ::core::option::Option::Some;

use crate::types::int;
use crate::types::uint;

const_trait_if! {
  #[feature("const_cmp")]
  impl<const N: usize> const PartialEq for int<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
      Self::const_eq(self, other)
    }
  }

  #[feature("const_cmp")]
  impl<const N: usize> const PartialEq for uint<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
      Self::const_eq(self, other)
    }
  }

  #[feature("const_cmp")]
  impl<const N: usize> const Eq for int<N> {}

  #[feature("const_cmp")]
  impl<const N: usize> const Eq for uint<N> {}

  #[feature("const_cmp")]
  impl<const N: usize> const PartialOrd for int<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(Ord::cmp(self, other))
    }
  }

  #[feature("const_cmp")]
  impl<const N: usize> const PartialOrd for uint<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(Ord::cmp(self, other))
    }
  }

  #[feature("const_cmp")]
  impl<const N: usize> const Ord for int<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
      Self::const_cmp(self, other)
    }
  }

  #[feature("const_cmp")]
  impl<const N: usize> const Ord for uint<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
      Self::const_cmp(self, other)
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::*;

  test!(test_eq, () => {
    assert!(<T as PartialEq>::eq(&T::P_0, &T::P_0));
    assert!(<T as PartialEq>::eq(&T::P_1, &T::P_1));
    assert!(<T as PartialEq>::eq(&T::P_2, &T::P_2));
    assert!(<T as PartialEq>::eq(&T::P_3, &T::P_3));

    assert!(<T as PartialEq>::eq(&T::MIN, &T::MIN));
    assert!(<T as PartialEq>::eq(&T::MAX, &T::MAX));

    assert!(<T as PartialEq>::ne(&T::P_0, &T::P_1));
    assert!(<T as PartialEq>::ne(&T::P_0, &T::P_2));
    assert!(<T as PartialEq>::ne(&T::P_0, &T::P_3));

    assert!(<T as PartialEq>::ne(&T::P_1, &T::P_0));
    assert!(<T as PartialEq>::ne(&T::P_1, &T::P_2));
    assert!(<T as PartialEq>::ne(&T::P_1, &T::P_3));

    assert!(<T as PartialEq>::ne(&T::P_2, &T::P_0));
    assert!(<T as PartialEq>::ne(&T::P_2, &T::P_1));
    assert!(<T as PartialEq>::ne(&T::P_2, &T::P_3));

    assert!(<T as PartialEq>::ne(&T::MIN, &T::MAX));
    assert!(<T as PartialEq>::ne(&T::MAX, &T::MIN));
  });

  test!(test_ord, () => {
    assert_eq!(<T as Ord>::cmp(&T::P_0, &T::P_0), Ordering::Equal);
    assert_eq!(<T as Ord>::cmp(&T::P_1, &T::P_1), Ordering::Equal);
    assert_eq!(<T as Ord>::cmp(&T::P_2, &T::P_2), Ordering::Equal);
    assert_eq!(<T as Ord>::cmp(&T::P_3, &T::P_3), Ordering::Equal);

    assert_eq!(<T as Ord>::cmp(&T::P_0, &T::P_1), Ordering::Less);
    assert_eq!(<T as Ord>::cmp(&T::P_1, &T::P_2), Ordering::Less);
    assert_eq!(<T as Ord>::cmp(&T::P_2, &T::P_3), Ordering::Less);

    assert_eq!(<T as Ord>::cmp(&T::P_3, &T::P_2), Ordering::Greater);
    assert_eq!(<T as Ord>::cmp(&T::P_2, &T::P_1), Ordering::Greater);
    assert_eq!(<T as Ord>::cmp(&T::P_1, &T::P_0), Ordering::Greater);

    assert_eq!(<T as Ord>::cmp(&T::MIN, &T::MAX), Ordering::Less);
    assert_eq!(<T as Ord>::cmp(&T::MAX, &T::MIN), Ordering::Greater);
  });

  test!(test_min_max, () => {
    assert_eq!(<T as Ord>::min(T::P_0, T::P_1), T::P_0);
    assert_eq!(<T as Ord>::min(T::P_1, T::P_2), T::P_1);
    assert_eq!(<T as Ord>::min(T::P_2, T::P_3), T::P_2);

    assert_eq!(<T as Ord>::max(T::P_0, T::P_1), T::P_1);
    assert_eq!(<T as Ord>::max(T::P_1, T::P_2), T::P_2);
    assert_eq!(<T as Ord>::max(T::P_2, T::P_3), T::P_3);

    assert_eq!(<T as Ord>::min(T::MIN, T::MAX), T::MIN);
    assert_eq!(<T as Ord>::min(T::MAX, T::MIN), T::MIN);

    assert_eq!(<T as Ord>::max(T::MIN, T::MAX), T::MAX);
    assert_eq!(<T as Ord>::max(T::MAX, T::MIN), T::MAX);
  });
}
