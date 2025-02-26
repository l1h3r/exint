macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::cmp::PartialEq for $crate::$name<N> {
      #[inline]
      fn eq(&self, other: &Self) -> bool {
        Self::const_eq(self, other)
      }
    }

    impl<const N: usize> ::core::cmp::Eq for $crate::$name<N> {}

    impl<const N: usize> ::core::cmp::PartialOrd for $crate::$name<N> {
      #[inline]
      fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
      }
    }

    impl<const N: usize> ::core::cmp::Ord for $crate::$name<N> {
      #[inline]
      fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        Self::const_cmp(self, other)
      }
    }
  };
}

implement!(int);
implement!(uint);
