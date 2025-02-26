macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::default::Default for $crate::$name<N> {
      #[doc = "Returns the default value of `0`"]
      #[inline]
      fn default() -> Self {
        Self::ZERO
      }
    }
  };
}

implement!(int);
implement!(uint);
