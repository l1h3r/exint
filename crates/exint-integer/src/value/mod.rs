pub(crate) trait Value {
  /// The number of bytes used to represent this value.
  const SIZE: usize;

  /// True if the value is an unsigned integer.
  const UINT: bool;
}

macro_rules! value {
  ($primitive:ty, $uint:literal) => {
    impl Value for $primitive {
      const SIZE: usize = ::core::mem::size_of::<$primitive>();
      const UINT: bool = $uint;
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
}

impl<const S: usize> Value for crate::uint<S> {
  const SIZE: usize = S;
  const UINT: bool = true;
}
