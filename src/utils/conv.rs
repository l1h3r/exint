use crate::error::TryFromIntError;
use crate::types::int;
use crate::types::uint;
use crate::utils::Cast;

pub(crate) trait TryConvert<T> {
  type Error;

  fn try_convert(self) -> ::core::result::Result<T, Self::Error>;
}

impl<const N: usize> TryConvert<u32> for uint<N> {
  type Error = TryFromIntError;

  #[inline]
  fn try_convert(self) -> ::core::result::Result<u32, Self::Error> {
    if ::core::matches!(N, 1 | 2 | 3 | 4) {
      ::core::result::Result::Ok(self.into_u32())
    } else if self > Cast::cast(u32::MAX) {
      ::core::result::Result::Err(TryFromIntError::new())
    } else {
      ::core::result::Result::Ok(self.into_u32())
    }
  }
}

impl<const N: usize> TryConvert<u32> for int<N> {
  type Error = TryFromIntError;

  #[inline]
  fn try_convert(self) -> ::core::result::Result<u32, Self::Error> {
    if self < Self::ZERO {
      ::core::result::Result::Err(TryFromIntError::new())
    } else if ::core::matches!(N, 1 | 2 | 3 | 4) {
      ::core::result::Result::Ok(self.into_u32())
    } else if self > Cast::cast(u32::MAX) {
      ::core::result::Result::Err(TryFromIntError::new())
    } else {
      ::core::result::Result::Ok(self.into_u32())
    }
  }
}

impl<const N: usize> TryConvert<usize> for uint<N> {
  type Error = TryFromIntError;

  #[inline]
  fn try_convert(self) -> ::core::result::Result<usize, Self::Error> {
    if N <= ::core::mem::size_of::<usize>() {
      ::core::result::Result::Ok(self.into_usize())
    } else if self > Cast::cast(usize::MAX) {
      ::core::result::Result::Err(TryFromIntError::new())
    } else {
      ::core::result::Result::Ok(self.into_usize())
    }
  }
}

macro_rules! implement_std {
  ($type:ty) => {
    impl TryConvert<u32> for $type {
      type Error = <$type as ::core::convert::TryInto<u32>>::Error;

      #[inline]
      fn try_convert(self) -> ::core::result::Result<u32, Self::Error> {
        ::core::convert::TryInto::try_into(self)
      }
    }
  };
  ($($type:ty)+) => {
    $(
      implement_std!($type);
    )+
  };
}

implement_std!(i8 i16 i32 i64 i128 isize);
implement_std!(u8 u16 u32 u64 u128 usize);

impl<const N: usize> TryConvert<uint<N>> for usize {
  type Error = TryFromIntError;

  #[inline]
  fn try_convert(self) -> ::core::result::Result<uint<N>, Self::Error> {
    if N >= ::core::mem::size_of::<usize>() {
      ::core::result::Result::Ok(uint::from_usize(self))
    } else if self > Cast::cast(uint::<N>::MAX) {
      ::core::result::Result::Err(TryFromIntError::new())
    } else {
      ::core::result::Result::Ok(uint::from_usize(self))
    }
  }
}
