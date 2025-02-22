macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::marker::Copy for $crate::$name<N> {}

    #[cfg(feature = "structural_match")]
    impl<const N: usize> ::core::marker::StructuralPartialEq for $crate::$name<N> {}

    #[cfg(feature = "unsized_const_params")]
    impl<const N: usize> ::core::marker::UnsizedConstParamTy for $crate::$name<N> {}

    #[cfg(feature = "adt_const_params")]
    impl<const N: usize> ::core::marker::ConstParamTy_ for $crate::$name<N> {}
  };
}

implement!(int);
implement!(uint);
