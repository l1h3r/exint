use crate::types::int;
use crate::types::uint;

macro_rules! try_from {
  ($from:ty, $into:ty, |$binding:ident| $block:block) => {
    impl ::core::convert::TryFrom<$from> for $into {
      type Error = $crate::error::TryFromIntError;

      #[inline]
      fn try_from($binding: $from) -> ::core::result::Result<Self, Self::Error> {
        $block
      }
    }
  };
}

macro_rules! impl_bool {
  ($name:ident) => {
    impl<const N: usize> ::core::convert::From<bool> for $crate::$name<N> {
      #[inline]
      fn from(other: bool) -> Self {
        Self::from_bool(other)
      }
    }
  };
}

macro_rules! impl_cast {
  (@impl From<$from:ty> for $into:ty) => {
    impl ::core::convert::From<$from> for $into {
      #[inline]
      fn from(other: $from) -> Self {
        $crate::utils::Cast::cast(other)
      }
    }
  };
  ($std:ident -> uint<$head:literal $(| $tail:literal)*>) => {
    impl_cast!(@impl From<$std> for uint<$head>);
    impl_upper_bounded!(@impl From<uint<$head>> for $std);
    impl_cast!($std -> uint<$($tail)|*>);
  };
  ($std:ident -> int<$head:literal $(| $tail:literal)*>) => {
    impl_cast!(@impl From<$std> for int<$head>);
    impl_range_bounded!(@impl From<int<$head>> for $std);
    impl_cast!($std -> int<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

macro_rules! impl_same {
  ($std:ident -> $lib:ty) => {
    impl_cast!(@impl From<$std> for $lib);
    impl_cast!(@impl From<$lib> for $std);
  };
}

macro_rules! impl_lower_bounded {
  (@impl From<$from:ty> for $into:ty) => {
    try_from!($from, $into, |other| {
      if other >= $crate::utils::Zero::ZERO {
        ::core::result::Result::Ok($crate::utils::Cast::cast(other))
      } else {
        ::core::result::Result::Err($crate::error::TryFromIntError::new())
      }
    });
  };
  ($std:ident -> uint<$head:literal $(| $tail:literal)*>) => {
    impl_lower_bounded!(@impl From<$std> for uint<$head>);
    impl_upper_bounded!(@impl From<uint<$head>> for $std);
    impl_lower_bounded!($std -> uint<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

macro_rules! impl_upper_bounded {
  (@impl From<$from:ty> for $into:ty) => {
    try_from!($from, $into, |other| {
      if other > $crate::utils::Cast::cast(Self::MAX) {
        ::core::result::Result::Err($crate::error::TryFromIntError::new())
      } else {
        ::core::result::Result::Ok($crate::utils::Cast::cast(other))
      }
    });
  };
  ($std:ident -> uint<$head:literal $(| $tail:literal)*>) => {
    impl_upper_bounded!(@impl From<$std> for uint<$head>);
    impl_cast!(@impl From<uint<$head>> for $std);
    impl_upper_bounded!($std -> uint<$($tail)|*>);
  };
  ($std:ident -> int<$head:literal $(| $tail:literal)*>) => {
    impl_upper_bounded!(@impl From<$std> for int<$head>);
    impl_lower_bounded!(@impl From<int<$head>> for $std);
    impl_upper_bounded!($std -> int<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

macro_rules! impl_range_bounded {
  (@impl From<$from:ty> for $into:ty) => {
    try_from!($from, $into, |other| {
      let max: $from = $crate::utils::Cast::cast(Self::MAX);
      let min: $from = $crate::utils::Cast::cast(Self::MIN);

      if other < min || other > max {
        ::core::result::Result::Err($crate::error::TryFromIntError::new())
      } else {
        ::core::result::Result::Ok($crate::utils::Cast::cast(other))
      }
    });
  };
  ($std:ident -> $name:ident<$head:literal $(| $tail:literal)*>) => {
    impl_range_bounded!(@impl From<$std> for $name<$head>);
    impl_cast!(@impl From<$name<$head>> for $std);
    impl_range_bounded!($std -> $name<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

// TODO: From<uint<S>> for int<S> via impl_upper_bounded
// TODO: From<int<S>> for uint<S> via impl_lower_bounded
// TODO: Platform-specific pointer sizes
macro_rules! implement {
  (int) => {
    // boolean to signed
    impl_bool!(int);

    // signed to signed - same size
    impl_same!(i8   -> int<1>);
    impl_same!(i16  -> int<2>);
    impl_same!(i32  -> int<4>);
    impl_same!(i64  -> int<8>);
    impl_same!(i128 -> int<16>);

    // signed (small) to signed (large)
    impl_cast!(i8   -> int<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(i16  -> int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(i32  -> int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(i64  -> int<9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(i128 -> int<32|64>);

    // signed (large) to signed (small)
    impl_range_bounded!(i16  -> int<1>);
    impl_range_bounded!(i32  -> int<1|2|3>);
    impl_range_bounded!(i64  -> int<1|2|3|4|5|6|7>);
    impl_range_bounded!(i128 -> int<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

    // unsigned (small) to signed (large)
    impl_cast!(u8   -> int<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u16  -> int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u32  -> int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u64  -> int<9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u128 -> int<32|64>);

    // unsigned (large) to signed (small)
    impl_upper_bounded!(u8   -> int<1>);
    impl_upper_bounded!(u16  -> int<1|2>);
    impl_upper_bounded!(u32  -> int<1|2|3|4>);
    impl_upper_bounded!(u64  -> int<1|2|3|4|5|6|7|8>);
    impl_upper_bounded!(u128 -> int<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16>);

    // signed (pointer) to signed (small)
    impl_range_bounded!(isize -> int<1>);

    // unsigned (pointer) to signed (small)
    impl_upper_bounded!(usize -> int<1>);
  };
  (uint) => {
    // boolean to unsigned
    impl_bool!(uint);

    // unsigned to unsigned - same size
    impl_same!(u8   -> uint<1>);
    impl_same!(u16  -> uint<2>);
    impl_same!(u32  -> uint<4>);
    impl_same!(u64  -> uint<8>);
    impl_same!(u128 -> uint<16>);

    // unsigned (small) to unsigned (large)
    impl_cast!(u8   -> uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u16  -> uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u32  -> uint<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u64  -> uint<9|10|11|12|13|14|15|16|32|64>);
    impl_cast!(u128 -> uint<32|64>);

    // unsigned (large) to unsigned (small)
    impl_upper_bounded!(u16  -> uint<1>);
    impl_upper_bounded!(u32  -> uint<1|2|3>);
    impl_upper_bounded!(u64  -> uint<1|2|3|4|5|6|7>);
    impl_upper_bounded!(u128 -> uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

    // signed (small) to unsigned (large)
    impl_lower_bounded!(i8   -> uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_lower_bounded!(i16  -> uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_lower_bounded!(i32  -> uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    impl_lower_bounded!(i64  -> uint<8|9|10|11|12|13|14|15|16|32|64>);
    impl_lower_bounded!(i128 -> uint<16|32|64>);

    // signed (large) to unsigned (small)
    impl_range_bounded!(i16  -> uint<1>);
    impl_range_bounded!(i32  -> uint<1|2>);
    impl_range_bounded!(i64  -> uint<1|2|3|4|5|6|7>);
    impl_range_bounded!(i128 -> uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

    // unsigned (pointer) to unsigned (small)
    impl_upper_bounded!(usize -> uint<1>);

    // signed (pointer) to unsigned (small)
    impl_range_bounded!(isize -> uint<1>);
  };
}

implement!(int);
implement!(uint);
