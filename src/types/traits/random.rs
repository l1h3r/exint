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
}

implement!(int);
implement!(uint);
