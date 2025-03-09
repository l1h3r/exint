use crate::macros::const_trait_impl;
use crate::utils::Index;
use crate::utils::SIGN;

/// A marker trait for integer types.
///
/// # Safety
///
/// Types implementing this trait must ensure they are valid representations of
/// integers with no padding or uninitialized bytes.
pub unsafe trait Uint: ::core::marker::Copy {
  /// Whether the type represents an unsigned integer.
  const UINT: bool;
}

macro_rules! implement_uint {
  ($type:ty, $uint:literal) => {
    unsafe impl Uint for $type {
      const UINT: bool = $uint;
    }
  };
  ($($type:ty)+, $uint:literal) => {
    $(
      implement_uint!($type, $uint);
    )+
  };
}

implement_uint!(i8 i16 i32 i64 i128 isize, false);
implement_uint!(u8 u16 u32 u64 u128 usize, true);

// -----------------------------------------------------------------------------
// Generic Constants
// -----------------------------------------------------------------------------

/// Supporting trait for constant integer values.
pub(crate) trait Consts {
  /// The size of this integer type in bytes.
  const SIZE: usize;
  /// The size of this integer type in bits.
  const BITS: u32;
  /// The largest unsigned value that can be represented by this integer type.
  const UMAX: Self;
  /// The smallest unsigned value that can be represented by this integer type.
  const UMIN: Self;
  /// The largest signed value that can be represented by this integer type.
  const SMAX: Self;
  /// The smallest signed value that can be represented by this integer type.
  const SMIN: Self;
}

impl<const N: usize> Consts for [u8; N] {
  const SIZE: usize = ::core::mem::size_of::<Self>();
  const BITS: u32 = (Self::SIZE as u32) << 3;

  const UMAX: Self = [u8::MAX; N];
  const UMIN: Self = [u8::MIN; N];

  const SMAX: Self = {
    let mut bytes: Self = Self::UMAX;
    bytes[Index::ZERO.msb::<N>()] ^= SIGN;
    bytes
  };

  const SMIN: Self = {
    let mut bytes: Self = Self::UMIN;
    bytes[Index::ZERO.msb::<N>()] |= SIGN;
    bytes
  };
}

// -----------------------------------------------------------------------------
// Cast Specialization
// -----------------------------------------------------------------------------

/// Supporting trait for casting between same-sized integer types.
///
/// This allows cheap conversion to/from generic integer types and their
/// equivalent built-in integer types.
#[cfg(feature = "min_specialization")]
#[const_trait]
pub(crate) trait Cast {
  /// The equivalent built-in unsigned integer type.
  type Uint;
  /// The equivalent built-in signed integer type.
  type Sint;
  /// Cast `self` to [`Uint`][Cast::Uint].
  fn ucast(self) -> Self::Uint;
  /// Cast `self` to [`Sint`][Cast::Sint].
  fn scast(self) -> Self::Sint;
}

const_trait_impl! {
  /// Implement for tuples to allow easier specialization of overflowing ops.
  #[cfg(feature = "min_specialization")]
  impl<T> const Cast for (T, bool)
  where
    T: ::core::marker::Copy + ~const Cast,
  {
    type Uint = (T::Uint, bool);
    type Sint = (T::Sint, bool);

    #[inline]
    fn ucast(self) -> Self::Uint {
      (self.0.ucast(), self.1)
    }

    #[inline]
    fn scast(self) -> Self::Sint {
      (self.0.scast(), self.1)
    }
  }
}

macro_rules! implement_cast {
  (from: Int<$size:literal>, uint: $uint:ty, sint: $sint:ty) => {
    // We should *always* be casting to same-sized types.
    const _: () = assert!($size == ::core::mem::size_of::<$uint>());
    const _: () = assert!($size == ::core::mem::size_of::<$sint>());

    const_trait_impl! {
      #[cfg(feature = "min_specialization")]
      impl const Cast for [u8; $size] {
        type Uint = $uint;
        type Sint = $sint;

        #[inline]
        fn ucast(self) -> Self::Uint {
          Self::Uint::from_ne_bytes(self)
        }

        #[inline]
        fn scast(self) -> Self::Sint {
          Self::Sint::from_ne_bytes(self)
        }
      }

      #[cfg(feature = "min_specialization")]
      impl const Cast for $uint {
        type Uint = [u8; $size];
        type Sint = ::core::convert::Infallible;

        #[inline]
        fn ucast(self) -> Self::Uint {
          Self::to_ne_bytes(self)
        }

        /// Use [`ucast`] when casting from unsigned built-in integers.
        ///
        /// # Panics
        ///
        /// This function unconditionally panics.
        ///
        /// [`ucast`]: Cast::ucast
        fn scast(self) -> Self::Sint {
          ::core::panic!()
        }
      }

      #[cfg(feature = "min_specialization")]
      impl const Cast for $sint {
        type Uint = ::core::convert::Infallible;
        type Sint = [u8; $size];

        /// Use [`scast`] when casting from signed built-in integers.
        ///
        /// # Panics
        ///
        /// This function unconditionally panics.
        ///
        /// [`scast`]: Cast::scast
        fn ucast(self) -> Self::Uint {
          ::core::panic!()
        }

        #[inline]
        fn scast(self) -> Self::Sint {
          Self::to_ne_bytes(self)
        }
      }
    }
  };
}

implement_cast!(from: Int<1>,  uint: u8,   sint: i8);
implement_cast!(from: Int<2>,  uint: u16,  sint: i16);
implement_cast!(from: Int<4>,  uint: u32,  sint: i32);
implement_cast!(from: Int<8>,  uint: u64,  sint: i64);
implement_cast!(from: Int<16>, uint: u128, sint: i128);

// -----------------------------------------------------------------------------
// Sext/Zext Specialization
// -----------------------------------------------------------------------------

/// Supporting trait for sign/zero-extending operations.
///
/// This allows cheap conversion from a generic integer type to the next largest
/// built-in integer type.
#[cfg(feature = "min_specialization")]
#[const_trait]
pub(crate) trait Exts {
  /// The next largest built-in unsigned integer type.
  type Uint;
  /// The next largest built-in signed integer type.
  type Sint;
  /// The size difference between this integer type and the next largest
  /// built-in integer type in bits.
  const UDIFF: u32;
  /// Zero-extend `self` to [`Uint`][Exts::Uint].
  fn zext(self) -> Self::Uint;
  /// Sign-extend `self` to [`Sint`][Exts::Sint].
  fn sext(self) -> Self::Sint;
}

/// Supporting trait for integer truncation operations.
///
/// This allows cheap conversion from a built-in integer type to a variety of
/// smaller generic integer types.
#[cfg(feature = "min_specialization")]
#[const_trait]
pub(crate) trait Trunc<Output> {
  /// Truncate `self` to an integer with a smaller size.
  fn trunc(self) -> Output;
}

macro_rules! implement_exts {
  (from: Int<$size:literal>, uint: $uint:ty, sint: $sint:ty) => {
    // We should *always* be extending to larger types.
    const _: () = assert!($size < ::core::mem::size_of::<$uint>());
    const _: () = assert!($size < ::core::mem::size_of::<$sint>());

    // Both signed and unsigned built-in types should be the same size.
    const _: () = assert!(::core::mem::size_of::<$uint>() == ::core::mem::size_of::<$sint>());

    #[cfg(feature = "min_specialization")]
    const _: () = {
      const PADDING: usize = ::core::mem::size_of::<$uint>() - $size;

      #[repr(C, packed)]
      struct Proxy {
        #[cfg(target_endian = "big")]
        padding: [u8; PADDING],
        integer: [u8; $size],
        #[cfg(target_endian = "little")]
        padding: [u8; PADDING],
      }

      impl Proxy {
        #[inline]
        const fn new(integer: [u8; $size]) -> Self {
          Self {
            integer,
            padding: [0; PADDING],
          }
        }

        #[inline]
        const fn get(self) -> [u8; $size] {
          self.integer
        }
      }

      const_trait_impl! {
        impl const Exts for [u8; $size] {
          type Uint = $uint;
          type Sint = $sint;

          const UDIFF: u32 = <$uint>::BITS - <[u8; $size] as Consts>::BITS;

          #[inline]
          fn zext(self) -> Self::Uint {
            // SAFETY: `Proxy` is the same size and layout as `Uint`.
            unsafe { ::core::mem::transmute::<Proxy, Self::Uint>(Proxy::new(self)) }
          }

          #[expect(clippy::cast_possible_wrap, reason = "Recognized sign-extend pattern")]
          #[inline]
          fn sext(self) -> Self::Sint {
            ((self.zext() as $sint) << Self::UDIFF) >> Self::UDIFF
          }
        }

        impl const Trunc<[u8; $size]> for $uint {
          #[inline]
          fn trunc(self) -> [u8; $size] {
            // SAFETY: `Self` is the same size and layout as `Proxy`.
            unsafe { ::core::mem::transmute::<Self, Proxy>(self) }.get()
          }
        }

        impl const Trunc<[u8; $size]> for $sint {
          #[expect(clippy::cast_sign_loss, reason = "Recognized truncate pattern")]
          #[inline]
          fn trunc(self) -> [u8; $size] {
            (self as $uint).trunc()
          }
        }
      }
    };
  };
}

implement_exts!(from: Int<3>,  uint: u32,  sint: i32);
implement_exts!(from: Int<5>,  uint: u64,  sint: i64);
implement_exts!(from: Int<6>,  uint: u64,  sint: i64);
implement_exts!(from: Int<7>,  uint: u64,  sint: i64);
implement_exts!(from: Int<9>,  uint: u128, sint: i128);
implement_exts!(from: Int<10>, uint: u128, sint: i128);
implement_exts!(from: Int<11>, uint: u128, sint: i128);
implement_exts!(from: Int<12>, uint: u128, sint: i128);
implement_exts!(from: Int<13>, uint: u128, sint: i128);
implement_exts!(from: Int<14>, uint: u128, sint: i128);
implement_exts!(from: Int<15>, uint: u128, sint: i128);
