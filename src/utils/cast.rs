use crate::llapi;
use crate::types::int;
use crate::types::uint;

pub(crate) trait Cast<U> {
  fn cast(self) -> U;
}

impl<const T: usize, const U: usize> Cast<int<U>> for int<T> {
  #[inline]
  fn cast(self) -> int<U> {
    int::from_ne_bytes(llapi::cast_bytes::<Self, T, U>(self.to_ne_bytes()))
  }
}

impl<const T: usize, const U: usize> Cast<uint<U>> for int<T> {
  #[inline]
  fn cast(self) -> uint<U> {
    uint::from_ne_bytes(llapi::cast_bytes::<Self, T, U>(self.to_ne_bytes()))
  }
}

impl<const T: usize, const U: usize> Cast<int<U>> for uint<T> {
  #[inline]
  fn cast(self) -> int<U> {
    int::from_ne_bytes(llapi::cast_bytes::<Self, T, U>(self.to_ne_bytes()))
  }
}

impl<const T: usize, const U: usize> Cast<uint<U>> for uint<T> {
  #[inline]
  fn cast(self) -> uint<U> {
    uint::from_ne_bytes(llapi::cast_bytes::<Self, T, U>(self.to_ne_bytes()))
  }
}

macro_rules! implement {
  ($type:ty, $uint:literal) => {
    const _: () = {
      const SIZE: usize = ::core::mem::size_of::<$type>();

      impl<const N: usize> Cast<$type> for int<N> {
        #[inline]
        fn cast(self) -> $type {
          <$type>::from_ne_bytes(llapi::cast_bytes::<Self, N, SIZE>(self.to_ne_bytes()))
        }
      }

      impl<const N: usize> Cast<$type> for uint<N> {
        #[inline]
        fn cast(self) -> $type {
          <$type>::from_ne_bytes(llapi::cast_bytes::<Self, N, SIZE>(self.to_ne_bytes()))
        }
      }

      impl<const N: usize> Cast<int<N>> for $type {
        #[inline]
        fn cast(self) -> int<N> {
          int::from_ne_bytes(llapi::cast_bytes::<Self, SIZE, N>(self.to_ne_bytes()))
        }
      }

      impl<const N: usize> Cast<uint<N>> for $type {
        #[inline]
        fn cast(self) -> uint<N> {
          uint::from_ne_bytes(llapi::cast_bytes::<Self, SIZE, N>(self.to_ne_bytes()))
        }
      }
    };
  };
  (sint $($type:ty)+) => {
    $(
      implement!($type, false);
    )+
  };
  (uint $($type:ty)+) => {
    $(
      implement!($type, true);
    )+
  };
}

implement!(sint i8 i16 i32 i64 i128 isize);
implement!(uint u8 u16 u32 u64 u128 usize);
