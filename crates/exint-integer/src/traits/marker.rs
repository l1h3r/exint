macro_rules! copy {
  ($name:ident) => {
    impl<const S: usize> ::core::marker::Copy for $name<S> {}
  };
}

pub(crate) use copy;
