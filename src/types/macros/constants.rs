macro_rules! constants {
  (int) => {
    #[doc = include_doc!(int, "BITS")]
    pub const BITS: u32 = $crate::types::uint::<N>::BITS;

    #[doc = include_doc!(int, "MAX")]
    pub const MAX: Self = $crate::types::uint::<N>::MAX.const_shr(1).cast_signed();

    #[doc = include_doc!(int, "MIN")]
    pub const MIN: Self = Self::MAX.const_not();
  };
  (uint) => {
    #[doc = include_doc!(uint, "BITS")]
    pub const BITS: u32 = Self::MAX.count_ones();

    #[doc = include_doc!(uint, "MAX")]
    pub const MAX: Self = Self::MIN.const_not();

    #[doc = include_doc!(uint, "MIN")]
    pub const MIN: Self = Self::ZERO;
  };
  ($outer:ident, $inner:ident) => {
    #[doc = include_doc!($outer, $inner, "BITS")]
    pub const BITS: u32 = $crate::types::$inner::<N>::BITS;

    #[doc = include_doc!($outer, $inner, "MAX")]
    pub const MAX: Self = Self($crate::types::$inner::<N>::MAX);

    #[doc = include_doc!($outer, $inner, "MIN")]
    pub const MIN: Self = Self($crate::types::$inner::<N>::MIN);
  };
}

pub(crate) use constants;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(test_bits, () => {
    assert_eq!(T::BITS, <T as IntExt>::BITS);
  });

  test!(@uint, test_min_uint, () => {
    assert_eq!(T::MIN, T::P_0);
  });

  test!(@uint, test_max_uint, () => {
    assert_eq!(T::MAX, T::P_2.wrapping_pow(T::BITS).wrapping_sub(T::P_1));
  });

  test!(@sint, test_min_sint, () => {
    assert_eq!(T::MIN, T::P_2.wrapping_pow(T::BITS - 1));
  });

  test!(@sint, test_max_sint, () => {
    assert_eq!(T::MAX, T::P_2.wrapping_pow(T::BITS - 1).wrapping_sub(T::P_1));
  });
}
