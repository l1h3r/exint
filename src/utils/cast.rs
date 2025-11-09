//! Internal Conversions
//!
//! Exists as a high-level wrapper around [`llapi::cast_bytes`].
//!
//! This is the preferred API for any type of internal integer conversion.
//!
//! [`llapi::cast_bytes`]: crate::llapi::cast_bytes

use crate::llapi::cast_bytes;
use crate::types::int;
use crate::types::uint;

// -----------------------------------------------------------------------------
// Cast
// -----------------------------------------------------------------------------

const_trait! {
  pub(crate) const trait Cast<T> {
    fn cast(self) -> T;
  }
}

// -----------------------------------------------------------------------------
// Boolean
// -----------------------------------------------------------------------------

impl<const N: usize> int<N> {
  #[inline]
  pub(crate) const fn from_bool(other: bool) -> Self {
    Self::from_u8(other as u8)
  }
}

impl<const N: usize> uint<N> {
  #[inline]
  pub(crate) const fn from_bool(other: bool) -> Self {
    Self::from_u8(other as u8)
  }
}

// -----------------------------------------------------------------------------
// Crate Types
// -----------------------------------------------------------------------------

const_trait_if! {
  #[feature("const_traits")]
  impl<const N: usize, const M: usize> const Cast<int<M>> for int<N> {
    #[inline]
    fn cast(self) -> int<M> {
      int::from_ne_bytes(cast_bytes::<false, N, M>(self.to_ne_bytes()))
    }
  }

  #[feature("const_traits")]
  impl<const N: usize, const M: usize> const Cast<uint<M>> for int<N> {
    #[inline]
    fn cast(self) -> uint<M> {
      int::<M>::cast_unsigned(Cast::cast(self))
    }
  }

  #[feature("const_traits")]
  impl<const N: usize, const M: usize> const Cast<int<M>> for uint<N> {
    #[inline]
    fn cast(self) -> int<M> {
      uint::<M>::cast_signed(Cast::cast(self))
    }
  }

  #[feature("const_traits")]
  impl<const N: usize, const M: usize> const Cast<uint<M>> for uint<N> {
    #[inline]
    fn cast(self) -> uint<M> {
      uint::from_ne_bytes(cast_bytes::<true, N, M>(self.to_ne_bytes()))
    }
  }
}

// -----------------------------------------------------------------------------
// Core Primitive Types
// -----------------------------------------------------------------------------

macro_rules! implement {
  ($type:ty, $uint:literal, $from:ident, $into:ident) => {
    const _: () = {
      const M: usize = ::core::mem::size_of::<$type>();

      impl<const N: usize> int<N> {
        #[doc(hidden)] // Only exposed for use with `int` macros and tests.
        #[inline]
        pub const fn $from(other: $type) -> Self {
          Self::from_ne_bytes(cast_bytes::<$uint, M, N>(other.to_ne_bytes()))
        }

        #[doc(hidden)] // Only exposed for use with `int` macros and tests.
        #[inline]
        pub const fn $into(self) -> $type {
          <$type>::from_ne_bytes(cast_bytes::<false, N, M>(self.to_ne_bytes()))
        }
      }

      impl<const N: usize> uint<N> {
        #[doc(hidden)] // Only exposed for use with `uint` macros and tests.
        #[inline]
        pub const fn $from(other: $type) -> Self {
          Self::from_ne_bytes(cast_bytes::<$uint, M, N>(other.to_ne_bytes()))
        }

        #[doc(hidden)] // Only exposed for use with `uint` macros and tests.
        #[inline]
        pub const fn $into(self) -> $type {
          <$type>::from_ne_bytes(cast_bytes::<true, N, M>(self.to_ne_bytes()))
        }
      }

      const_trait_if! {
        #[feature("const_traits")]
        impl<const N: usize> const Cast<$type> for int<N> {
          #[inline]
          fn cast(self) -> $type {
            int::$into(self)
          }
        }

        #[feature("const_traits")]
        impl<const N: usize> const Cast<$type> for uint<N> {
          #[inline]
          fn cast(self) -> $type {
            uint::$into(self)
          }
        }

        #[feature("const_traits")]
        impl<const N: usize> const Cast<int<N>> for $type {
          #[inline]
          fn cast(self) -> int<N> {
            int::$from(self)
          }
        }

        #[feature("const_traits")]
        impl<const N: usize> const Cast<uint<N>> for $type {
          #[inline]
          fn cast(self) -> uint<N> {
            uint::$from(self)
          }
        }
      }
    };
  };
}

implement!(u8,    true, from_u8,    into_u8);
implement!(u16,   true, from_u16,   into_u16);
implement!(u32,   true, from_u32,   into_u32);
implement!(u64,   true, from_u64,   into_u64);
implement!(u128,  true, from_u128,  into_u128);
implement!(usize, true, from_usize, into_usize);

implement!(i8,    false, from_i8,    into_i8);
implement!(i16,   false, from_i16,   into_i16);
implement!(i32,   false, from_i32,   into_i32);
implement!(i64,   false, from_i64,   into_i64);
implement!(i128,  false, from_i128,  into_i128);
implement!(isize, false, from_isize, into_isize);
