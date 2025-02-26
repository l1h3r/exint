use crate::types::int;
use crate::types::uint;

pub(crate) trait Zero {
  const ZERO: Self;
}

impl<const N: usize> Zero for int<N> {
  const ZERO: Self = Self::ZERO;
}

impl<const N: usize> Zero for uint<N> {
  const ZERO: Self = Self::ZERO;
}

macro_rules! implement {
  ($type:ty) => {
    impl Zero for $type {
      const ZERO: Self = 0;
    }
  };
  ($($type:ty)+) => {
    $(
      implement!($type);
    )+
  };
}

implement!(i8 i16 i32 i64 i128 isize);
implement!(u8 u16 u32 u64 u128 usize);
