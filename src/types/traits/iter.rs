use ::core::iter::Iterator;
use ::core::iter::Product;
use ::core::iter::Sum;

use crate::types::Saturating;
use crate::types::Wrapping;
use crate::types::int;
use crate::types::uint;

macro_rules! implement_sum {
  ($type:ty) => {
    impl<const N: usize> Sum for $type {
      fn sum<I>(iter: I) -> Self
      where
        I: Iterator<Item = Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }

    impl<'a, const N: usize> Sum<&'a $type> for $type {
      fn sum<I>(iter: I) -> Self
      where
        I: Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }
  };
}

macro_rules! implement_product {
  ($type:ty) => {
    impl<const N: usize> Product for $type {
      fn product<I>(iter: I) -> Self
      where
        I: Iterator<Item = Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }

    impl<'a, const N: usize> Product<&'a $type> for $type {
      fn product<I>(iter: I) -> Self
      where
        I: Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }
  };
}

#[cfg(feature = "step_trait")]
macro_rules! implement_step {
  (@core) => {
    #[inline]
    fn forward(start: Self, n: usize) -> Self {
      // In debug builds, trigger a panic on overflow.
      // This should optimize completely out in release builds.
      if Self::forward_checked(start, n).is_none() {
        let _: Self = Self::MAX + Self::ONE;
      }

      // Do wrapping math to allow e.g. `Step::forward(-128i8, 255)`.
      start.wrapping_add(Self::from_usize(n))
    }

    #[inline]
    fn backward(start: Self, n: usize) -> Self {
      // In debug builds, trigger a panic on overflow.
      // This should optimize completely out in release builds.
      if Self::backward_checked(start, n).is_none() {
        let _: Self = Self::MIN - Self::ONE;
      }

      // Do wrapping math to allow e.g. `Step::backward(127i8, 255)`.
      start.wrapping_sub(Self::from_usize(n))
    }
  };
  (@methods int) => {
    #[inline]
    unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        start
          .checked_add_unsigned(uint::from_usize(n))
          .unwrap_unchecked()
      }
    }

    #[inline]
    unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        start
          .checked_sub_unsigned(uint::from_usize(n))
          .unwrap_unchecked()
      }
    }

    #[inline]
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
      if *start <= *end {
        if N > ::core::mem::size_of::<usize>() {
          let steps: usize = end
            .into_isize()
            .wrapping_sub(start.into_isize())
            .cast_unsigned();

          return (steps, Some(steps));
        }

        if let Some(result) = end.checked_sub(*start) {
          if let Ok(steps) = TryFrom::try_from(result) {
            (steps, Some(steps))
          } else {
            (usize::MAX, None)
          }
        } else {
          (usize::MAX, None)
        }
      } else {
        (0, None)
      }
    }

    #[inline]
    fn forward_checked(start: Self, n: usize) -> Option<Self> {
      if N > ::core::mem::size_of::<usize>() {
        return start.checked_add(Self::from_usize(n));
      }

      let Ok(n) = <uint<N> as TryFrom<_>>::try_from(n) else {
        return None;
      };

      let wrapped: Self = start.wrapping_add(n.cast_signed());

      if wrapped >= start {
        Some(wrapped)
      } else {
        None
      }
    }

    #[inline]
    fn backward_checked(start: Self, n: usize) -> Option<Self> {
      if N > ::core::mem::size_of::<usize>() {
        return start.checked_sub(Self::from_usize(n));
      }

      let Ok(n) = <uint<N> as TryFrom<_>>::try_from(n) else {
        return None;
      };

      let wrapped: Self = start.wrapping_sub(n.cast_signed());

      if wrapped <= start {
        Some(wrapped)
      } else {
        None
      }
    }
  };
  (@methods uint) => {
    #[inline]
    unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { start.unchecked_add(Self::from_usize(n)) }
    }

    #[inline]
    unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe { start.unchecked_sub(Self::from_usize(n)) }
    }

    #[inline]
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
      if *start <= *end {
        let Ok(steps) = TryFrom::try_from(*end - *start) else {
          return (usize::MAX, None);
        };

        (steps, Some(steps))
      } else {
        (0, None)
      }
    }

    #[inline]
    fn forward_checked(start: Self, n: usize) -> Option<Self> {
      let Ok(n) = TryFrom::try_from(n) else {
        return None;
      };

      start.checked_add(n)
    }

    #[inline]
    fn backward_checked(start: Self, n: usize) -> Option<Self> {
      let Ok(n) = TryFrom::try_from(n) else {
        return None;
      };

      start.checked_sub(n)
    }
  };
  ($type:ident) => {
    #[cfg(feature = "step_trait")]
    const _: () = {
      use ::core::iter::Step;
      use ::core::option::Option;
      use ::core::option::Option::None;
      use ::core::option::Option::Some;
      use ::core::result::Result::Ok;

      use $crate::utils::TryFrom;

      // Note: This is just here to show the feature callout in rustdoc
      #[cfg(feature = "step_trait")]
      impl<const N: usize> Step for $type<N> {
        implement_step!(@core);
        implement_step!(@methods $type);
      }
    };

    #[cfg(feature = "trusted_step")]
    unsafe impl<const N: usize> ::core::iter::TrustedStep for $type<N> {}
  };
}

implement_sum!(int<N>);
implement_sum!(uint<N>);
implement_sum!(Saturating<int<N>>);
implement_sum!(Saturating<uint<N>>);
implement_sum!(Wrapping<int<N>>);
implement_sum!(Wrapping<uint<N>>);

implement_product!(int<N>);
implement_product!(uint<N>);
implement_product!(Saturating<int<N>>);
implement_product!(Saturating<uint<N>>);
implement_product!(Wrapping<int<N>>);
implement_product!(Wrapping<uint<N>>);

#[cfg(feature = "step_trait")]
implement_step!(int);

#[cfg(feature = "step_trait")]
implement_step!(uint);

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::*;

  test!(test_sum, () => {
    assert_eq!(T::A_5.iter().sum::<T>(), T::P_15);
    assert_eq!(T::A_5.iter().skip(1).sum::<T>(), T::P_14);
    assert_eq!(T::A_5.iter().skip(2).sum::<T>(), T::P_12);
    assert_eq!(T::A_5.iter().skip(3).sum::<T>(), T::P_9);
    assert_eq!(T::A_5.iter().skip(4).sum::<T>(), T::P_5);
    assert_eq!(T::A_5.iter().skip(5).sum::<T>(), T::P_0);

    assert_eq!(T::A_5.iter().take(0).sum::<T>(), T::P_0);
    assert_eq!(T::A_5.iter().take(1).sum::<T>(), T::P_1);
    assert_eq!(T::A_5.iter().take(2).sum::<T>(), T::P_3);
    assert_eq!(T::A_5.iter().take(3).sum::<T>(), T::P_6);
    assert_eq!(T::A_5.iter().take(4).sum::<T>(), T::P_10);
    assert_eq!(T::A_5.iter().take(5).sum::<T>(), T::P_15);
  });

  test!(test_product, () => {
    assert_eq!(T::A_5.iter().product::<T>(), T::P_120);
    assert_eq!(T::A_5.iter().skip(1).product::<T>(), T::P_120);
    assert_eq!(T::A_5.iter().skip(2).product::<T>(), T::P_60);
    assert_eq!(T::A_5.iter().skip(3).product::<T>(), T::P_20);
    assert_eq!(T::A_5.iter().skip(4).product::<T>(), T::P_5);
    assert_eq!(T::A_5.iter().skip(5).product::<T>(), T::P_1);

    assert_eq!(T::A_5.iter().take(0).product::<T>(), T::P_1);
    assert_eq!(T::A_5.iter().take(1).product::<T>(), T::P_1);
    assert_eq!(T::A_5.iter().take(2).product::<T>(), T::P_2);
    assert_eq!(T::A_5.iter().take(3).product::<T>(), T::P_6);
    assert_eq!(T::A_5.iter().take(4).product::<T>(), T::P_24);
    assert_eq!(T::A_5.iter().take(5).product::<T>(), T::P_120);
  });
}
