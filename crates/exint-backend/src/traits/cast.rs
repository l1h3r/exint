use ::core::marker::Copy;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Cast between two integers types of the same size.
#[const_trait]
pub(crate) unsafe trait Cast {
  type Uint;
  type Sint;
  fn ucast(self) -> Self::Uint;
  fn scast(self) -> Self::Sint;
}

// -----------------------------------------------------------------------------
// Implementations
// -----------------------------------------------------------------------------

// Tuple implementation to make overflowing operations easier to specialize
unsafe impl<T, U> const Cast for (T, U)
where
  T: Copy + ~const Cast,
  U: Copy,
{
  type Uint = (T::Uint, U);
  type Sint = (T::Sint, U);

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
  (from: Int<$size:literal>, uint: $uint:ty, sint: $sint:ty) => {
    unsafe impl const Cast for $crate::types::Int<$size> {
      type Uint = $uint;
      type Sint = $sint;

      #[inline]
      fn ucast(self) -> Self::Uint {
        // SAFETY: `Int<$size>` is the same size and layout as `$uint`
        unsafe { $crate::utils::transmute::<Self, Self::Uint>(self) }
      }

      #[inline]
      fn scast(self) -> Self::Sint {
        // SAFETY: `Int<$size>` is the same size and layout as `$sint`
        unsafe { $crate::utils::transmute::<Self, Self::Sint>(self) }
      }
    }

    unsafe impl const Cast for $uint {
      type Uint = $crate::types::Int<$size>;
      type Sint = $crate::types::Int<$size>;

      #[inline]
      fn ucast(self) -> Self::Uint {
        // SAFETY: `$uint` is the same size and layout as `Int<$size>`
        unsafe { $crate::utils::transmute::<Self, Self::Uint>(self) }
      }

      fn scast(self) -> Self::Sint {
        ::core::unreachable!("nope")
      }
    }

    unsafe impl const Cast for $sint {
      type Uint = $crate::types::Int<$size>;
      type Sint = $crate::types::Int<$size>;

      fn ucast(self) -> Self::Uint {
        ::core::unreachable!("nope")
      }

      #[inline]
      fn scast(self) -> Self::Sint {
        // SAFETY: `$sint` is the same size and layout as `Int<$size>`
        unsafe { $crate::utils::transmute::<Self, Self::Sint>(self) }
      }
    }
  };
}

implement!(from: Int<1>,  uint: u8,   sint: i8);
implement!(from: Int<2>,  uint: u16,  sint: i16);
implement!(from: Int<4>,  uint: u32,  sint: i32);
implement!(from: Int<8>,  uint: u64,  sint: i64);
implement!(from: Int<16>, uint: u128, sint: i128);
