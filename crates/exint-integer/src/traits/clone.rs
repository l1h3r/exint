macro_rules! clone {
  ($name:ident) => {
    impl<const S: usize> ::core::clone::Clone for $name<S> {
      #[inline]
      fn clone(&self) -> Self {
        *self
      }
    }
  };
}

pub(crate) use clone;
