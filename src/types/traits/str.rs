use ::core::result::Result;
use ::core::str::FromStr;

use crate::error::ParseIntError;
use crate::types::Saturating;
use crate::types::Wrapping;
use crate::types::int;
use crate::types::uint;

const_trait_if! {
  #[feature("const_convert")]
  impl<const N: usize> const FromStr for int<N> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
      Self::from_str_radix(src, 10)
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const FromStr for uint<N> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
      Self::from_str_radix(src, 10)
    }
  }
}

const_trait_if! {
  #[feature("const_convert")]
  impl<T: [const] FromStr> const FromStr for Saturating<T> {
    type Err = <T as FromStr>::Err;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
      FromStr::from_str(src).map(Self)
    }
  }

  #[feature("const_convert")]
  impl<T: [const] FromStr> const FromStr for Wrapping<T> {
    type Err = <T as FromStr>::Err;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
      FromStr::from_str(src).map(Self)
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::*;

  test!(test_from_str, () => {
    assert_eq!(<T as FromStr>::from_str(T::MIN_STR), Ok(T::MIN));
    assert_eq!(<T as FromStr>::from_str(T::MAX_STR), Ok(T::MAX));
  });
}
