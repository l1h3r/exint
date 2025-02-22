macro_rules! default {
  ($name:ident) => {
    impl<const S: usize> ::core::default::Default for $name<S> {
      fn default() -> Self {
        panic!("Default::default")
      }
    }
  };
}

pub(crate) use default;
