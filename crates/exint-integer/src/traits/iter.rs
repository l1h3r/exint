macro_rules! product {
  ($name:ident) => {
    impl<const S: usize> ::core::iter::Product for $name<S> {
      fn product<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }

    impl<'a, const S: usize> ::core::iter::Product<&'a $name<S>> for $name<S> {
      fn product<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ONE, |a, b| a * b)
      }
    }
  };
}

macro_rules! sum {
  ($name:ident) => {
    impl<const S: usize> ::core::iter::Sum for $name<S> {
      fn sum<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }

    impl<'a, const S: usize> ::core::iter::Sum<&'a $name<S>> for $name<S> {
      fn sum<I>(iter: I) -> Self
      where
        I: ::core::iter::Iterator<Item = &'a Self>,
      {
        iter.fold(Self::ZERO, |a, b| a + b)
      }
    }
  };
}

macro_rules! step {
  ($name:ident) => {
    #[cfg(feature = "step_trait")]
    #[doc(cfg(feature = "step_trait"))]
    impl<const S: usize> ::core::iter::Step for $name<S> {
      fn forward(start: Self, n: usize) -> Self {
        panic!("Step::forward")
      }

      fn backward(start: Self, n: usize) -> Self {
        panic!("Step::backward")
      }

      unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
        panic!("Step::forward_unchecked")
      }

      unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
        panic!("Step::backward_unchecked")
      }

      fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        panic!("Step::steps_between")
      }

      fn forward_checked(start: Self, n: usize) -> Option<Self> {
        panic!("Step::forward_checked")
      }

      fn backward_checked(start: Self, n: usize) -> Option<Self> {
        panic!("Step::backward_checked")
      }
    }
  };
}

macro_rules! trusted_step {
  ($name:ident) => {
    #[cfg(feature = "trusted_step")]
    #[doc(cfg(feature = "trusted_step"))]
    unsafe impl<const S: usize> ::core::iter::TrustedStep for $name<S> {}
  };
}

pub(crate) use product;
pub(crate) use sum;
pub(crate) use step;
pub(crate) use trusted_step;
