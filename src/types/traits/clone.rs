use ::core::clone::Clone;

use crate::types::int;
use crate::types::uint;

const_trait_if! {
  #[feature("const_clone")]
  impl<const N: usize> const Clone for int<N> {
    #[inline]
    fn clone(&self) -> Self {
      *self
    }
  }

  #[feature("const_clone")]
  impl<const N: usize> const Clone for uint<N> {
    #[inline]
    fn clone(&self) -> Self {
      *self
    }
  }
}
