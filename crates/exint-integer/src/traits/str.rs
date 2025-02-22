macro_rules! from_str {
  ($name:ident) => {
    impl<const S: usize> ::core::str::FromStr for $name<S> {
      type Err = $crate::errors::ParseIntError;

      #[inline]
      fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
      }
    }
  };
}

pub(crate) use from_str;
