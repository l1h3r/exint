#[allow(unused_macro_rules, reason = "Step branches are not not always used")]
macro_rules! implement {
  ($outer:ident<$inner:ident<N>>) => {
    implement!(@core $crate::$outer<$crate::$inner<N>>);
  };
  ($type:ident) => {
    implement!(@core $crate::$type<N>);
    implement!(@step $type);
  };
  (@core $type:ty) => {
    impl<const N: usize> ::core::iter::Sum for $type {
      fn sum<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }

    impl<'a, const N: usize> ::core::iter::Sum<&'a $type> for $type {
      fn sum<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }

    impl<const N: usize> ::core::iter::Product for $type {
      fn product<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }

    impl<'a, const N: usize> ::core::iter::Product<&'a $type> for $type {
      fn product<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }
  };
  (@step_methods core) => {
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
  (@step_methods int) => {
    #[inline]
    unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        start.checked_add_unsigned($crate::uint::from_usize(n)).unwrap_unchecked()
      }
    }

    #[inline]
    unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        start.checked_sub_unsigned($crate::uint::from_usize(n)).unwrap_unchecked()
      }
    }

    #[inline]
    fn steps_between(_start: &Self, _end: &Self) -> (usize, ::core::option::Option<usize>) {
      ::core::panic!("TODO - Step::steps_between")
    }

    #[inline]
    fn forward_checked(_start: Self, _n: usize) -> ::core::option::Option<Self> {
      ::core::panic!("TODO - Step::forward_checked")
    }

    #[inline]
    fn backward_checked(_start: Self, _n: usize) -> ::core::option::Option<Self> {
      ::core::panic!("TODO - Step::backward_checked")
    }
  };
  (@step_methods uint) => {
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
    fn steps_between(start: &Self, end: &Self) -> (usize, ::core::option::Option<usize>) {
      if *start <= *end {
        match <Self as $crate::utils::TryConvert<usize>>::try_convert(*end - *start) {
          ::core::result::Result::Ok(steps) => (steps, ::core::option::Option::Some(steps)),
          ::core::result::Result::Err(_) => (usize::MAX, ::core::option::Option::None),
        }
      } else {
        (0, ::core::option::Option::None)
      }
    }

    #[inline]
    fn forward_checked(start: Self, n: usize) -> ::core::option::Option<Self> {
      match <usize as $crate::utils::TryConvert<Self>>::try_convert(n) {
        ::core::result::Result::Ok(n) => start.checked_add(n),
        ::core::result::Result::Err(_) => ::core::option::Option::None,
      }
    }

    #[inline]
    fn backward_checked(start: Self, n: usize) -> ::core::option::Option<Self> {
      match <usize as $crate::utils::TryConvert<Self>>::try_convert(n) {
        ::core::result::Result::Ok(n) => start.checked_sub(n),
        ::core::result::Result::Err(_) => ::core::option::Option::None,
      }
    }
  };
  (@step $type:ident) => {
    #[cfg(feature = "step_trait")]
    impl<const N: usize> ::core::iter::Step for $crate::$type<N> {
      implement!(@step_methods core);
      implement!(@step_methods $type);
    }

    #[cfg(feature = "trusted_step")]
    unsafe impl<const N: usize> ::core::iter::TrustedStep for $crate::$type<N> {}
  };
}

implement!(int);
implement!(uint);

implement!(Saturating<int<N>>);
implement!(Saturating<uint<N>>);

#[cfg(feature = "strict_overflow_ops")]
implement!(Strict<int<N>>);
#[cfg(feature = "strict_overflow_ops")]
implement!(Strict<uint<N>>);

implement!(Wrapping<int<N>>);
implement!(Wrapping<uint<N>>);
