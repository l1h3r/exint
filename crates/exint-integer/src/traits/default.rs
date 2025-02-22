macro_rules! default {
  ($name:ident) => {
    impl<const S: usize> ::core::default::Default for $name<S> {
      #[doc = "Returns the default value of `0`"]
      #[inline]
      fn default() -> Self {
        Self::ZERO
      }
    }
  };
}

pub(crate) use default;
