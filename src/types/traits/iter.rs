macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::iter::Sum for $crate::$name<N> {
      fn sum<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }

    impl<'a, const N: usize> ::core::iter::Sum<&'a $crate::$name<N>> for $crate::$name<N> {
      fn sum<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }

    impl<const N: usize> ::core::iter::Product for $crate::$name<N> {
      fn product<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }

    impl<'a, const N: usize> ::core::iter::Product<&'a $crate::$name<N>> for $crate::$name<N> {
      fn product<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }

    #[cfg(feature = "step_trait")]
    impl<const N: usize> ::core::iter::Step for $crate::$name<N> {
      fn forward(_start: Self, _n: usize) -> Self {
        ::core::todo!("Step::forward")
      }

      fn backward(_start: Self, _n: usize) -> Self {
        ::core::todo!("Step::backward")
      }

      unsafe fn forward_unchecked(_start: Self, _n: usize) -> Self {
        ::core::todo!("Step::forward_unchecked")
      }

      unsafe fn backward_unchecked(_start: Self, _n: usize) -> Self {
        ::core::todo!("Step::backward_unchecked")
      }

      fn steps_between(_start: &Self, _end: &Self) -> (usize, ::core::option::Option<usize>) {
        ::core::todo!("Step::steps_between")
      }

      fn forward_checked(_start: Self, _n: usize) -> ::core::option::Option<Self> {
        ::core::todo!("Step::forward_checked")
      }

      fn backward_checked(_start: Self, _n: usize) -> ::core::option::Option<Self> {
        ::core::todo!("Step::backward_checked")
      }
    }

    #[cfg(feature = "trusted_step")]
    unsafe impl<const N: usize> ::core::iter::TrustedStep for $crate::$name<N> {}
  };
}

implement!(int);
implement!(uint);
