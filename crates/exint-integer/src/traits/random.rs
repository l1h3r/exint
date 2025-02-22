macro_rules! random {
  ($name:ident) => {
    #[cfg(feature = "random")]
    #[doc(cfg(feature = "random"))]
    impl<const S: usize> ::core::random::Random for $name<S> {
      fn random(source: &mut (impl ::core::random::RandomSource + ?Sized)) -> Self {
        let mut bytes: [u8; S] = [0x00; S];

        source.fill_bytes(&mut bytes);

        Self::from_ne_bytes(bytes)
      }
    }
  };
}

pub(crate) use random;
