macro_rules! implement {
  ($name:ident) => {
    // Not an issue - we implement Clone via Copy.
    #[expect(clippy::expl_impl_clone_on_copy)]
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
