macro_rules! implement {
  ($name:ident) => {
    #[expect(clippy::expl_impl_clone_on_copy, reason = "This is Clone via Copy")]
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
