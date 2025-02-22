macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::clone::Clone for $crate::$name<N> {
      #[inline]
      fn clone(&self) -> Self {
        *self
      }
    }
  };
}

implement!(int);
implement!(uint);
