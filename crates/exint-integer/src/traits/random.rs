macro_rules! random {
  ($name:ident) => {
    #[cfg(feature = "random")]
    #[doc(cfg(feature = "random"))]
    impl<const S: usize> ::core::random::Random for $name<S> {
      fn random(source: &mut (impl ::core::random::RandomSource + ?Sized)) -> Self {
        panic!("Random::random")
      }
    }
  };
}

pub(crate) use random;
