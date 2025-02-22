macro_rules! clone {
  ($name:ident) => {
    impl<const S: usize> ::core::clone::Clone for $name<S> {
      fn clone(&self) -> Self {
        panic!("Clone::clone")
      }
    }
  };
}

pub(crate) use clone;
