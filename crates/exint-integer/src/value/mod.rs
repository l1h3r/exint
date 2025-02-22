use ::core::option::Option;

pub(crate) trait Value {
  /// The number of bytes used to represent this type.
  const SIZE: usize;

  /// True if the type is an unsigned integer.
  const UINT: bool;

  /// The representation of `0` for this type.
  const ZERO: Self;

  /// Convert this type to an unsigned 32-bit integer.
  fn try_u32(self) -> Option<u32>;
}

macro_rules! value {
  ($primitive:ty, $uint:literal) => {
    impl Value for $primitive {
      const SIZE: usize = ::core::mem::size_of::<$primitive>();
      const UINT: bool = $uint;
      const ZERO: Self = 0;

      #[inline]
      fn try_u32(self) -> Option<u32> {
        ::core::convert::TryInto::try_into(self).ok()
      }
    }
  };
}

value!(u8,    true);
value!(u16,   true);
value!(u32,   true);
value!(u64,   true);
value!(u128,  true);
value!(usize, true);

value!(i8,    false);
value!(i16,   false);
value!(i32,   false);
value!(i64,   false);
value!(i128,  false);
value!(isize, false);

impl<const S: usize> Value for crate::int<S> {
  const SIZE: usize = S;
  const UINT: bool = false;
  const ZERO: Self = Self::from_u8(0);

  #[inline]
  fn try_u32(self) -> Option<u32> {
    Self::try_into_u32(self).ok()
  }
}

impl<const S: usize> Value for crate::uint<S> {
  const SIZE: usize = S;
  const UINT: bool = true;
  const ZERO: Self = Self::from_u8(0);

  #[inline]
  fn try_u32(self) -> Option<u32> {
    Self::try_into_u32(self).ok()
  }
}
