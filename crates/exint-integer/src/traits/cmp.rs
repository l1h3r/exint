macro_rules! partial_eq {
  ($name:ident) => {
    impl<const S: usize> ::core::cmp::PartialEq for $name<S> {
      #[inline]
      fn eq(&self, other: &Self) -> bool {
        Self::const_eq(self, other)
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
      #[inline]
      fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        Some(<Self as ::core::cmp::Ord>::cmp(self, other))
      }
    }
  };
}

macro_rules! ord {
  ($name:ident) => {
    impl<const S: usize> ::core::cmp::Ord for $name<S> {
      #[inline]
      fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        Self::const_cmp(self, other)
      }
    }
  };
}

pub(crate) use eq;
pub(crate) use ord;
pub(crate) use partial_eq;
pub(crate) use partial_ord;
