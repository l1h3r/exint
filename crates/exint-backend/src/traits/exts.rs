use ::core::marker::Sized;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

#[const_trait]
pub(crate) unsafe trait Exts: Sized {
  type Uint;
  type Sint;
  type Repr;

  /// The number of bytes used to represent this type.
  const SIZE: usize = ::core::mem::size_of::<Self>();

  /// The number of bits used to represent this type.
  const BITS: u32 = (Self::SIZE as u32) << 3;

  /// The maximum representable unsigned value for this type.
  const UMAX: Self;

  /// The minimum representable unsigned value for this type.
  const UMIN: Self;

  const UDIFF: u32;
  const UMASK: Self::Uint;
  const USIZE: usize;

  /// Zero extend `Self` to [`Uint`][Exts::Uint].
  fn zext(self) -> Self::Uint;

  /// Sign extend `Self` to [`Sint`][Exts::Sint].
  fn sext(self) -> Self::Sint;
}

/// Truncate the bytes of `Self` to a smaller size integer.
#[const_trait]
pub(crate) unsafe trait Trunc<Output> {
  fn trunc(self) -> Output;
}

// -----------------------------------------------------------------------------
// Implementations
// -----------------------------------------------------------------------------

macro_rules! implement {
  (from: Int<$size:literal>, uint: $uint:ty, sint: $sint:ty) => {
    const _: () = assert!($size < ::core::mem::size_of::<$uint>());

    unsafe impl const Exts for $crate::types::Int<$size> {
      type Uint = $uint;
      type Sint = $sint;
      type Repr = $crate::types::Pad<$size, { Self::USIZE - $size }>;

      const UMAX: Self = [u8::MAX; $size];
      const UMIN: Self = [u8::MIN; $size];

      const UDIFF: u32 = <$uint>::BITS - Self::BITS;
      const UMASK: Self::Uint = Self::UMAX.zext();
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

    unsafe impl const Trunc<$crate::types::Int<$size>> for $uint {
      #[inline]
      fn trunc(self) -> $crate::types::Int<$size> {
        // SAFETY: `Self` is the same size and layout as `Int::Repr`.
        let repr: <$crate::types::Int<$size> as Exts>::Repr = unsafe {
          ::core::intrinsics::transmute::<Self, _>(self)
        };

        repr.get()
      }
    }

    unsafe impl const Trunc<$crate::types::Int<$size>> for $sint {
      #[inline]
      fn trunc(self) -> $crate::types::Int<$size> {
        (self as $uint).trunc()
      }
    }
  };
}

implement!(from: Int<3>,  uint: u32,  sint: i32);
implement!(from: Int<5>,  uint: u64,  sint: i64);
implement!(from: Int<6>,  uint: u64,  sint: i64);
implement!(from: Int<7>,  uint: u64,  sint: i64);
implement!(from: Int<9>,  uint: u128, sint: i128);
implement!(from: Int<10>, uint: u128, sint: i128);
implement!(from: Int<11>, uint: u128, sint: i128);
implement!(from: Int<12>, uint: u128, sint: i128);
implement!(from: Int<13>, uint: u128, sint: i128);
implement!(from: Int<14>, uint: u128, sint: i128);
implement!(from: Int<15>, uint: u128, sint: i128);
