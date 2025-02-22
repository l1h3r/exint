macro_rules! constants {
  (uint) => {
    pub const MIN: Self = Self::ZERO;

    pub const MAX: Self = Self::MIN.const_not();

    pub const BITS: u32 = Self::MAX.count_ones();
  };
  (int) => {
    pub const MIN: Self = Self::MAX.const_not();

    pub const MAX: Self = <$crate::uint<S>>::MAX.const_shr(1).to_int();

    pub const BITS: u32 = <$crate::uint<S>>::BITS;
  };
}

pub(crate) use constants;
