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
  ($outer:ident<T>) => {
    impl<T: ::core::str::FromStr> ::core::str::FromStr for $crate::$outer<T> {
      type Err = <T as ::core::str::FromStr>::Err;

      #[inline]
      fn from_str(src: &str) -> ::core::result::Result<Self, Self::Err> {
        <T as ::core::str::FromStr>::from_str(src).map(Self)
      }
    }
  };
}

implement!(int);
implement!(uint);

implement!(Saturating<T>);
#[cfg(feature = "strict_overflow_ops")]
implement!(Strict<T>);
implement!(Wrapping<T>);
