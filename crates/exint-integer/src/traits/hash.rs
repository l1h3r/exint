macro_rules! hash {
  ($name:ident) => {
    impl<const S: usize> ::core::hash::Hash for $name<S> {
      fn hash<H>(&self, state: &mut H)
      where
        H: ::core::hash::Hasher,
      {
        panic!("Hash::hash")
      }

      fn hash_slice<H>(data: &[Self], state: &mut H)
      where
        H: ::core::hash::Hasher,
      {
        panic!("Hash::hash_slice")
      }
    }
  };
}

pub(crate) use hash;
