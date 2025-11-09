use ::core::default::Default;

use crate::types::int;
use crate::types::uint;

const_trait_if! {
  #[feature("const_default")]
  impl<const N: usize> const Default for int<N> {
    #[doc = "Returns the default value of `0`"]
    #[inline]
    fn default() -> Self {
      Self::ZERO
    }
  }

  #[feature("const_default")]
  impl<const N: usize> const Default for uint<N> {
    #[doc = "Returns the default value of `0`"]
    #[inline]
    fn default() -> Self {
      Self::ZERO
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

  test!(test_default, () => {
    assert_eq!(<T as Default>::default(), T::P_0);
  });
}
