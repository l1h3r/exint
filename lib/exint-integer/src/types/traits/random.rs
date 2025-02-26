macro_rules! implement {
  ($name:ident) => {
    #[cfg(feature = "random")]
    impl<const N: usize> ::core::random::Random for $crate::$name<N> {
      fn random(source: &mut (impl ::core::random::RandomSource + ?::core::marker::Sized)) -> Self {
        let mut bytes: [u8; N] = [0; N];

        source.fill_bytes(&mut bytes);

        Self::from_ne_bytes(bytes)
      }
    }
  };
  ($outer:ident<T>) => {
    #[cfg(feature = "random")]
    impl<T: ::core::random::Random> ::core::random::Random for $crate::$outer<T> {
      #[inline]
      fn random(source: &mut (impl ::core::random::RandomSource + ?::core::marker::Sized)) -> Self {
        Self(<T as ::core::random::Random>::random(source))
      }
    }
  };
}

implement!(int);
implement!(uint);

implement!(Saturating<T>);
#[cfg(feature = "strict_overflow_ops")]
implement!(Strict<T>);
implement!(Wrapping<T>);
