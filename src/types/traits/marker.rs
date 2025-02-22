macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::marker::Copy for $crate::$name<N> {}
  };
}

implement!(int);
implement!(uint);
