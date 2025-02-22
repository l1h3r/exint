macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::str::FromStr for $crate::$name<N> {
      type Err = $crate::error::ParseIntError;

      #[inline]
      fn from_str(src: &str) -> ::core::result::Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
      }
    }
  };
}

implement!(int);
implement!(uint);
