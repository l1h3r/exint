//! Common traits, for internal use only.

use crate::consts::SIGN;
use crate::types::Integer;
use crate::utils::msb_index;

/// Supporting trait for constant integer values.
#[const_trait]
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

/// Supporting trait for casting between same-sized integer types.
///
/// This allows cheap conversion to/from generic integer types and their
/// equivalent built-in integer types.
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

/// Supporting trait for sign/zero-extending operations.
///
/// This allows cheap conversion from a generic integer type to the next largest
/// built-in integer type.
#[const_trait]
pub(crate) trait Exts {
  /// The next largest built-in unsigned integer type.
  type Uint;
  /// The next largest built-in signed integer type.
  type Sint;
  /// Intermediate representation of this integer type during sign/zero-extension.
  type Repr;
  /// The size difference between this integer type and the next largest
  /// built-in integer type in bits.
  const UDIFF: u32;
  /// The size of the next largest built-in integer type in bits.
  const USIZE: usize;
  /// Zero-extend `self` to [`Uint`][Exts::Uint].
  fn zext(self) -> Self::Uint;
  /// Sign-extend `self` to [`Sint`][Exts::Sint].
  fn sext(self) -> Self::Sint;
}

/// Supporting trait for integer truncation operations.
///
/// This allows cheap conversion from a built-in integer type to a variety of
/// smaller generic integer types.
#[const_trait]
pub(crate) trait Trunc<Output> {
  /// Truncate `self` to an integer with a smaller size.
  fn trunc(self) -> Output;
}

/// Supporting trait for SIMD-accelerated operations.
#[const_trait]
pub(crate) trait SimdExt {
  type Simd;
  /// Convert a generic integer type into the equivalent SIMD vector.
  fn into_simd(self) -> Self::Simd;
  /// Convert a SIMD vector into a generic integer type.
  fn from_simd(simd: Self::Simd) -> Self;
}

impl<const S: usize> const Consts for Integer<S> {
  const SIZE: usize = ::core::mem::size_of::<Self>();
  const BITS: u32 = (Self::SIZE as u32) << 3;

  const UMAX: Self = [0xFF; S];
  const UMIN: Self = [0x00; S];

  const SMAX: Self = {
    let mut bytes: Self = Self::UMAX;
    bytes[msb_index::<{ S }>()] ^= SIGN;
    bytes
  };

  const SMIN: Self = {
    let mut bytes: Self = Self::UMIN;
    bytes[msb_index::<{ S }>()] |= SIGN;
    bytes
  };
}

/// Implement for tuples to allow easier specialization of overflowing ops.
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

macro_rules! implement {
  (@cast => from: Int<$size:literal>, uint: $uint:ty, sint: $sint:ty) => {
    impl const Cast for Integer<$size> {
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

    impl const Cast for $uint {
      type Uint = Integer<$size>;
      type Sint = !;

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

    impl const Cast for $sint {
      type Uint = !;
      type Sint = Integer<$size>;

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
  };
  (@exts => from: Int<$size:literal>, uint: $uint:ty, sint: $sint:ty) => {
    // We should *always* be extending to larger types.
    const _: () = assert!($size < ::core::mem::size_of::<$uint>());
    const _: () = assert!($size < ::core::mem::size_of::<$sint>());

    // Both signed and unsigned built-in types should be the same size.
    const _: () = assert!(::core::mem::size_of::<$uint>() == ::core::mem::size_of::<$sint>());

    impl const Exts for Integer<$size> {
      type Uint = $uint;
      type Sint = $sint;
      type Repr = $crate::types::Padding<$size, { Self::USIZE - $size }>;

      const UDIFF: u32 = <$uint>::BITS - Self::BITS;
      const USIZE: usize = ::core::mem::size_of::<$uint>();

      #[inline]
      fn zext(self) -> Self::Uint {
        // SAFETY: `Repr` is the same size and layout as `Uint`.
        unsafe {
          ::core::intrinsics::transmute::<Self::Repr, Self::Uint>(Self::Repr::new(self))
        }
      }

      #[inline]
      fn sext(self) -> Self::Sint {
        ((self.zext() as $sint) << Self::UDIFF) >> Self::UDIFF
      }
    }

    impl const Trunc<Integer<$size>> for $uint {
      #[inline]
      fn trunc(self) -> Integer<$size> {
        // SAFETY: `Self` is the same size and layout as `Integer::Repr`.
        let repr: <Integer<$size> as Exts>::Repr = unsafe {
          ::core::intrinsics::transmute::<Self, _>(self)
        };

        repr.get()
      }
    }

    impl const Trunc<Integer<$size>> for $sint {
      #[inline]
      fn trunc(self) -> Integer<$size> {
        (self as $uint).trunc()
      }
    }
  };
  (@simd => from: Int<$size:literal>, repr: [$type:ty; $lanes:literal]) => {
    const _: () = assert!($size % ::core::mem::size_of::<$type>() == 0);
    const _: () = assert!($lanes * ::core::mem::size_of::<$type>() == $size);

    impl const SimdExt for Integer<$size> {
      type Simd = ::core::simd::Simd<$type, $lanes>;

      #[inline]
      fn into_simd(self) -> Self::Simd {
        // SAFETY: `Self` is the same size and layout as `[$type; $lanes]`.
        Self::Simd::from_array(unsafe {
          ::core::intrinsics::transmute(self)
        })
      }

      #[inline]
      fn from_simd(simd: Self::Simd) -> Self {
        // SAFETY: `[$type; $lanes]` is the same size and layout as `Self`.
        unsafe {
          ::core::intrinsics::transmute(simd.to_array())
        }
      }
    }
  };
}

implement!(@cast => from: Int<1>,  uint: u8,   sint: i8);
implement!(@cast => from: Int<2>,  uint: u16,  sint: i16);
implement!(@cast => from: Int<4>,  uint: u32,  sint: i32);
implement!(@cast => from: Int<8>,  uint: u64,  sint: i64);
implement!(@cast => from: Int<16>, uint: u128, sint: i128);

implement!(@exts => from: Int<3>,  uint: u32,  sint: i32);
implement!(@exts => from: Int<5>,  uint: u64,  sint: i64);
implement!(@exts => from: Int<6>,  uint: u64,  sint: i64);
implement!(@exts => from: Int<7>,  uint: u64,  sint: i64);
implement!(@exts => from: Int<9>,  uint: u128, sint: i128);
implement!(@exts => from: Int<10>, uint: u128, sint: i128);
implement!(@exts => from: Int<11>, uint: u128, sint: i128);
implement!(@exts => from: Int<12>, uint: u128, sint: i128);
implement!(@exts => from: Int<13>, uint: u128, sint: i128);
implement!(@exts => from: Int<14>, uint: u128, sint: i128);
implement!(@exts => from: Int<15>, uint: u128, sint: i128);

implement!(@simd => from: Int<32>,  repr: [u64; 4]);
implement!(@simd => from: Int<64>,  repr: [u64; 8]);
implement!(@simd => from: Int<128>, repr: [u64; 16]);
implement!(@simd => from: Int<256>, repr: [u64; 32]);
implement!(@simd => from: Int<512>, repr: [u64; 64]);
