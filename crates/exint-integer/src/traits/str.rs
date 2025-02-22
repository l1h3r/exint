macro_rules! from_str {
  ($name:ident) => {
    impl<const S: usize> ::core::str::FromStr for $name<S> {
      type Err = $crate::errors::ParseIntError;

      fn from_str(src: &str) -> Result<Self, Self::Err> {
        panic!("FromStr::from_str")
      }
    }
  };
}

pub(crate) use from_str;
