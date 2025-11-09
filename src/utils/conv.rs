//! Internal Conversions
//!
//! Exists as a workaround for the lack of generic conversion from/into built-in types.
//!
//! - conversion into u32 used to support the Shl/Shr implementations
//! - conversion from/into usize used to support the Step trait implementation

use ::core::marker::Sized;
use ::core::matches;
use ::core::mem::size_of;
use ::core::result::Result;
use ::core::result::Result::Err;
use ::core::result::Result::Ok;

use crate::error::TryFromIntError;
use crate::types::int;
use crate::types::uint;
use crate::utils::Cast;

const_trait! {
  pub(crate) const trait TryFrom<T>: Sized {
    type Error;
    fn try_from(other: T) -> Result<Self, Self::Error>;
  }
}

const_trait_if! {
  #[feature("const_convert")]
  impl<const N: usize> const TryFrom<int<N>> for usize {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: int<N>) -> Result<Self, Self::Error> {
      if value.is_negative() {
        Err(TryFromIntError::new())
      } else if N <= size_of::<usize>() || value <= Cast::cast(usize::MAX) {
        Ok(value.into_usize())
      } else {
        Err(TryFromIntError::new())
      }
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const TryFrom<uint<N>> for usize {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: uint<N>) -> Result<Self, Self::Error> {
      if N <= size_of::<Self>() || value <= Cast::cast(Self::MAX) {
        Ok(value.into_usize())
      } else {
        Err(TryFromIntError::new())
      }
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const TryFrom<usize> for uint<N> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
      if N >= size_of::<usize>() || value <= Cast::cast(Self::MAX) {
        Ok(Self::from_usize(value))
      } else {
        Err(TryFromIntError::new())
      }
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const TryFrom<int<N>> for u32 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: int<N>) -> Result<Self, Self::Error> {
      if value.is_negative() {
        Err(TryFromIntError::new())
      } else if matches!(N, 1..=4) {
        Ok(value.into_u32())
      } else if value > Cast::cast(u32::MAX) {
        Err(TryFromIntError::new())
      } else {
        Ok(value.into_u32())
      }
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const TryFrom<uint<N>> for u32 {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: uint<N>) -> Result<Self, Self::Error> {
      if matches!(N, 1..=4) {
        Ok(value.into_u32())
      } else if value > Cast::cast(u32::MAX) {
        Err(TryFromIntError::new())
      } else {
        Ok(value.into_u32())
      }
    }
  }
}

macro_rules! implement {
  ($($type:ty)+) => {
    const_trait_if! {
      $(
        #[feature("const_convert")]
        impl const TryFrom<$type> for u32 {
          type Error = <u32 as ::core::convert::TryFrom<$type>>::Error;

          #[inline]
          fn try_from(value: $type) -> Result<Self, Self::Error> {
            ::core::convert::TryFrom::try_from(value)
          }
        }
      )+
    }
  };
}

implement!(i8 i16 i32 i64 i128 isize);
implement!(u8 u16 u32 u64 u128 usize);
