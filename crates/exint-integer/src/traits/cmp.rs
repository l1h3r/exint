macro_rules! partial_eq {
  ($name:ident) => {
    impl<const S: usize> ::core::cmp::PartialEq for $name<S> {
      fn eq(&self, other: &Self) -> bool {
        panic!("PartialEq::eq")
      }
    }
  };
}

macro_rules! eq {
  ($name:ident) => {
    impl<const S: usize> ::core::cmp::Eq for $name<S> {}
  };
}

macro_rules! partial_ord {
  ($name:ident) => {
    impl<const S: usize> ::core::cmp::PartialOrd for $name<S> {
      fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        panic!("PartialOrd::partial_cmp")
      }
    }
  };
}

macro_rules! ord {
  ($name:ident) => {
    impl<const S: usize> ::core::cmp::Ord for $name<S> {
      fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        panic!("Ord::cmp")
      }
    }
  };
}

pub(crate) use partial_eq;
pub(crate) use eq;
pub(crate) use partial_ord;
pub(crate) use ord;
